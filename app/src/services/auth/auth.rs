use std::sync::Arc;

use async_graphql::Context;
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
    TransactionError, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    configs::env::Env,
    dtos::app_state::AppState,
    models::{
        recovery_code, session, user,
        user_dtos::{UserLoginRequest, UserLoginResponse, UserSignupRequest, UserSignupResponse},
    },
    services::crypto::{
        derive_kek, encrypt_dek, generate_dek, generate_recovery_keys, generate_session_token,
        hash_master_password, hash_recovery_code, verify_master_password,
    },
    utils::error::{AppError, AppResult},
};

fn generate_recovery_keys_for_dek(
    dek: &[u8; 32],
    user_id: &Uuid,
    env_variables: &Arc<Env>,
) -> AppResult<(Vec<recovery_code::ActiveModel>, Vec<String>)> {
    let recovery_keys = generate_recovery_keys(env_variables.recovery_keys_count);

    let mut recovery_code_entities: Vec<recovery_code::ActiveModel> = vec![];

    for recovery_code in recovery_keys.iter() {
        let hash = hash_recovery_code(&recovery_code);

        let recovery_kek = derive_kek(&recovery_code)?;

        let encrypted_encrypted_dek = encrypt_dek(dek, &recovery_kek).unwrap();

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

pub async fn signup(
    ctx: &Context<'_>,
    request: UserSignupRequest,
) -> AppResult<UserSignupResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;
    let env_variables = &app_state.env_variables;

    let email = request.email;
    let master_password = request.master_password;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email.to_string()))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if user.is_some() {
        return Err(AppError::Conflict("User Already Exists".to_string()));
    }

    let kek = derive_kek(&master_password)?;
    let dek = generate_dek();

    let encrypted_dek = encrypt_dek(&dek, &kek)?;

    let user_id = Uuid::new_v4();
    let master_password_hash = hash_master_password(&master_password)?;

    let user_entity = user::ActiveModel {
        id: Set(user_id),
        email: Set(email.clone()),
        master_password_hash: Set(master_password_hash),
        encrypted_dek: Set(encrypted_dek),
        ..Default::default()
    };

    let (recovery_code_entities, recovery_keys) =
        generate_recovery_keys_for_dek(&dek, &user_id, env_variables)?;

    let session_token = generate_session_token();
    let expires_at = Utc::now() + Duration::minutes(env_variables.session_expire_minutes);

    let session = session::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id.clone()),
        expires_at: Set(expires_at),
        token: Set(session_token.clone()),
        ..Default::default()
    };

    db_connection
        .transaction(move |txn| {
            Box::pin(async move {
                user_entity.insert(txn).await?;

                recovery_code::Entity::insert_many(recovery_code_entities)
                    .exec(txn)
                    .await?;

                session::Entity::insert(session).exec(txn).await?;
                Ok(())
            })
        })
        .await
        .map_err(|e: TransactionError<DbErr>| AppError::Database(e.to_string()))?;

    ctx.insert_http_header(
        "Set-Cookie",
        format!(
            "session_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Expires={}",
            session_token,
            expires_at.format("%a, %d %b %Y %T GMT")
        ),
    );

    return Ok(UserSignupResponse {
        id: user_id,
        recovery_keys,
    });
}

pub async fn login(ctx: &Context<'_>, request: UserLoginRequest) -> AppResult<UserLoginResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let db_connection = &app_state.database_connection;
    let env_variables = &app_state.env_variables;

    let email = request.email;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email.to_string()))
        .one(db_connection.as_ref())
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    if user.is_none() {
        return Err(AppError::NotFound("User Not Found".to_string()));
    }

    let user = user.unwrap();

    let user_master_password_hash = user.master_password_hash;
    let request_master_password = request.master_password;

    if !verify_master_password(&request_master_password, &user_master_password_hash)? {
        return Err(AppError::Authorization(
            "Invalid Master Password".to_string(),
        ));
    }

    let session_token = generate_session_token();
    let expires_at = Utc::now() + Duration::minutes(env_variables.session_expire_minutes);

    session::Entity::insert(session::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id.clone()),
        expires_at: Set(expires_at),
        token: Set(session_token.clone()),
        ..Default::default()
    })
    .exec(db_connection.as_ref())
    .await
    .map_err(|e| AppError::Database(e.to_string()))?;

    ctx.insert_http_header(
        "Set-Cookie",
        format!(
            "session_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Expires={}",
            session_token,
            expires_at.format("%a, %d %b %Y %T GMT")
        ),
    );

    return Ok(UserLoginResponse {
        message: "Login Successful".to_string(),
    });
}
