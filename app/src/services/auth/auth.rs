use std::sync::Arc;

use async_graphql::Context;
use axum::http::header;
use chrono::{Duration, Utc};
use r2d2::Pool;
use redis::{Client, Commands};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, TransactionError,
    TransactionTrait,
};
use serde_json::to_string;
use uuid::Uuid;

use crate::{
    configs::env::Env,
    dtos::{
        app_state::AppState,
        graphql_context::GraphQLContext,
        response::{GraphqlGenericResponse, GraphqlResponse},
    },
    models::{
        recovery_code, user,
        user_dtos::{
            ChangeMasterPasswordRequest, CheckRecoveryCodeValidityRequest, RecoveryAccountRequest,
            RecoveryKeyResponse, UserLoginRequest, UserRedisSession, UserSignupRequest,
            UserSignupResponse,
        },
    },
    services::crypto::{self, decrypt_dek, verify_master_password},
    utils::error::{AppError, AppResult},
};

fn generate_recovery_keys_for_dek(
    dek: &[u8; 32],
    user_id: &Uuid,
    env_variables: &Arc<Env>,
) -> AppResult<(Vec<recovery_code::ActiveModel>, Vec<String>)> {
    let recovery_keys = crypto::generate_recovery_keys(env_variables.recovery_keys_count);

    let mut recovery_code_entities: Vec<recovery_code::ActiveModel> = vec![];

    for recovery_code in recovery_keys.iter() {
        let hash = crypto::hash_recovery_code(&recovery_code);

        let recovery_kek = crypto::derive_kek(&recovery_code)?;

        let encrypted_encrypted_dek = crypto::encrypt_dek(dek, &recovery_kek).unwrap();

        let recovery_code_entity = recovery_code::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id.clone()),
            code_hash: Set(hash),
            encrypted_dek: Set(encrypted_encrypted_dek),
            ..Default::default()
        };

        recovery_code_entities.push(recovery_code_entity);
    }

    Ok((recovery_code_entities, recovery_keys))
}

fn generate_and_save_session(
    user_redis_session: UserRedisSession,
    redis_pool_manager: &Arc<Pool<Client>>,
    env_variables: Arc<Env>,
    ctx: &Context<'_>,
) -> AppResult<()> {
    let session_token = crypto::generate_session_token();
    let expires_at = Utc::now() + Duration::minutes(env_variables.session_expire_minutes);

    let user_redis_session_str =
        to_string(&user_redis_session).map_err(|e| AppError::Internal(e.to_string()))?;

    let mut redis_connection = redis_pool_manager
        .get()
        .map_err(|e| AppError::Database(e.to_string()))?;

    redis_connection
        .set_ex::<String, String, ()>(
            session_token.to_string(),
            user_redis_session_str,
            (env_variables.session_expire_minutes as u64) * 60,
        )
        .map_err(|e| AppError::Database(e.to_string()))
        .unwrap();

    ctx.insert_http_header(
        header::SET_COOKIE,
        format!(
            "session_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Expires={}",
            session_token,
            expires_at.format("%a, %d %b %Y %T GMT")
        ),
    );

    Ok(())
}

pub async fn signup(
    ctx: &Context<'_>,
    request: UserSignupRequest,
) -> AppResult<GraphqlResponse<UserSignupResponse>> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;
    let redis_pool_manager = &app_state.redis_pool_manager;
    let env_variables = &app_state.env_variables;

    let email = request.email;
    let master_password = request.master_password;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if user.is_some() {
        return Err(AppError::Conflict("User Already Exists".to_string()));
    }

    let kek = crypto::derive_kek(&master_password)?;
    let dek = crypto::generate_dek();

    let encrypted_dek = crypto::encrypt_dek(&dek, &kek)?;

    let user_id = Uuid::new_v4();
    let master_password_hash = crypto::hash_master_password(&master_password)?;

    let user_entity = user::ActiveModel {
        id: Set(user_id),
        email: Set(email.clone()),
        master_password_hash: Set(master_password_hash),
        encrypted_dek: Set(encrypted_dek),
        ..Default::default()
    };

    let (recovery_code_entities, recovery_keys) =
        generate_recovery_keys_for_dek(&dek, &user_id, env_variables)?;

    db_connection
        .transaction(move |txn| {
            Box::pin(async move {
                user_entity.insert(txn).await?;

                recovery_code::Entity::insert_many(recovery_code_entities)
                    .exec(txn)
                    .await?;

                Ok(())
            })
        })
        .await
        .map_err(|e: TransactionError<DbErr>| AppError::Database(e.to_string()))?;

    generate_and_save_session(
        UserRedisSession {
            id: user_id,
            dek: dek.to_vec(),
            email,
        },
        redis_pool_manager,
        env_variables.clone(),
        ctx,
    )?;

    return Ok(GraphqlResponse::<UserSignupResponse> {
        success: true,
        message: "Signup Successful".to_string(),
        data: UserSignupResponse {
            recovery_keys,
            id: user_id,
        },
    });
}

pub async fn login(
    ctx: &Context<'_>,
    request: UserLoginRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;
    let redis_pool_manager = &app_state.redis_pool_manager;
    let env_variables = &app_state.env_variables;

    let email = request.email;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::NotFound("User Not Found".to_string()))?;

    let user_master_password_hash = user.master_password_hash;
    let request_master_password = request.master_password;

    if !crypto::verify_master_password(&request_master_password, &user_master_password_hash)? {
        return Err(AppError::Authorization(
            "Invalid Master Password".to_string(),
        ));
    }

    let kek = crypto::derive_kek(&request_master_password)?;
    let dek = crypto::decrypt_dek(&user.encrypted_dek, &kek)?;

    generate_and_save_session(
        UserRedisSession {
            id: user.id,
            dek,
            email: user.email,
        },
        redis_pool_manager,
        env_variables.clone(),
        ctx,
    )?;

    return Ok(GraphqlGenericResponse {
        success: true,
        message: "Login Successful".to_string(),
    });
}

pub async fn logout(ctx: &Context<'_>, _: &UserRedisSession) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let mut redis_connection = app_state
        .redis_pool_manager
        .get()
        .map_err(|_| AppError::Internal("Failed to get redis connection from pool".to_string()))?;

    let gql_ctx = ctx
        .data::<Arc<GraphQLContext>>()
        .map_err(|_| AppError::Internal("GraphQL Context is not passed".to_string()))?;

    let session_token = gql_ctx
        .session_token
        .as_ref()
        .ok_or(AppError::Authorization(
            "Session token is missing".to_string(),
        ))?;

    redis_connection
        .del::<String, ()>(session_token.to_string())
        .map_err(|_| AppError::Authorization("Session token is invalid or expired".to_string()))?;

    ctx.insert_http_header(
        header::SET_COOKIE,
        "session_token=; HttpOnly; Secure; SameSite=Strict; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT"
            .to_string(),
    );

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Logout Successful".to_string(),
    })
}

pub async fn check_recovery_code_validity(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: CheckRecoveryCodeValidityRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;

    let user_id = user_redis_session.id;

    let recovery_code = request.recovery_code;

    let recovery_code_hash = crypto::hash_recovery_code(&recovery_code);

    let recovery_code_entity = recovery_code::Entity::find()
        .filter(recovery_code::Column::CodeHash.eq(recovery_code_hash))
        .filter(recovery_code::Column::UserId.eq(user_id))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::NotFound("Recovery Code Not Found".to_string()))?;

    if recovery_code_entity.used {
        return Err(AppError::Conflict("Recovery Code Already Used".to_string()));
    }

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Recovery Code is Valid".to_string(),
    })
}

pub async fn generate_recovery_keys(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
) -> AppResult<GraphqlResponse<RecoveryKeyResponse>> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;

    let user_id = user_redis_session.id;
    let dek_u8_32: [u8; 32] = user_redis_session
        .dek
        .clone()
        .try_into()
        .map_err(|_| AppError::Crypto("Unable to convert DEK to [u8; 32]".to_string()))?;

    let recovery_code_entities = recovery_code::Entity::find()
        .filter(recovery_code::Column::UserId.eq(user_id))
        .filter(recovery_code::Column::Used.eq(false))
        .all(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if recovery_code_entities.len() != 0 {
        return Err(AppError::Conflict(
            "Recovery Keys Already Generated".to_string(),
        ));
    }

    let (recovery_code_entities, recovery_keys) =
        generate_recovery_keys_for_dek(&dek_u8_32, &user_id, &app_state.env_variables)?;

    recovery_code::Entity::insert_many(recovery_code_entities)
        .exec(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(GraphqlResponse::<RecoveryKeyResponse> {
        success: true,
        message: "Recovery Keys Generated".to_string(),
        data: RecoveryKeyResponse { recovery_keys },
    })
}

pub async fn recover_account(
    ctx: &Context<'_>,
    request: RecoveryAccountRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;

    let email = request.email;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&email))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::NotFound("User Not Found".to_string()))?;

    let recovery_code = request.recovery_code;
    let recovery_code_hash = crypto::hash_recovery_code(&recovery_code);

    let recovery_code_entity = recovery_code::Entity::find()
        .filter(recovery_code::Column::CodeHash.eq(recovery_code_hash))
        .filter(recovery_code::Column::UserId.eq(user.id))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::NotFound("Recovery Code Not Found".to_string()))?;

    if recovery_code_entity.used {
        return Err(AppError::Conflict("Recovery Code Already Used".to_string()));
    }

    let new_master_password = request.new_master_password;
    let new_kek = crypto::derive_kek(&new_master_password)?;
    let new_master_password_hash = crypto::hash_master_password(&new_master_password)?;

    let recovery_kek = crypto::derive_kek(&recovery_code)?;
    let dek = decrypt_dek(&recovery_code_entity.encrypted_dek, &recovery_kek)?;

    let encrypted_dek = crypto::encrypt_dek(&dek, &new_kek)?;

    let mut user_model: user::ActiveModel = user.into();
    user_model.master_password_hash = Set(new_master_password_hash);
    user_model.encrypted_dek = Set(encrypted_dek);
    user_model.updated_at = Set(Utc::now());

    let mut recovery_code_entity: recovery_code::ActiveModel = recovery_code_entity.into();
    recovery_code_entity.used = Set(true);
    recovery_code_entity.updated_at = Set(Utc::now());

    db_connection
        .transaction(move |txn| {
            Box::pin(async move {
                user_model.update(txn).await?;
                recovery_code_entity.update(txn).await?;
                Ok(())
            })
        })
        .await
        .map_err(|e: TransactionError<DbErr>| AppError::Database(e.to_string()))?;

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Account Recovered Successfully".to_string(),
    })
}

pub async fn change_master_password(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: ChangeMasterPasswordRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;

    let user_id = user_redis_session.id;

    let old_master_password = request.old_master_password;

    let user = user::Entity::find()
        .filter(user::Column::Id.eq(user_id))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or(AppError::Authorization(
            "Invalid Master Password".to_string(),
        ))?;

    if !verify_master_password(&old_master_password, &user.master_password_hash)? {
        return Err(AppError::Authorization(
            "Invalid Master Password".to_string(),
        ));
    }

    let encrypted_dek = &user.encrypted_dek;
    let new_master_password = request.new_master_password;
    let new_master_password_hash = crypto::hash_master_password(&new_master_password)?;

    let old_kek = crypto::derive_kek(&old_master_password)?;
    let new_kek = crypto::derive_kek(&new_master_password)?;

    let dek = crypto::decrypt_dek(encrypted_dek, &old_kek)?;
    let encrypted_dek = crypto::encrypt_dek(&dek, &new_kek)?;

    let mut user_model: user::ActiveModel = user.into();
    user_model.master_password_hash = Set(new_master_password_hash);
    user_model.encrypted_dek = Set(encrypted_dek);
    user_model.updated_at = Set(Utc::now());

    user_model
        .update(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Master Password Changed Successfully".to_string(),
    })
}
