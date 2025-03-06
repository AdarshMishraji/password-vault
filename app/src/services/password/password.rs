use std::sync::Arc;

use async_graphql::Context;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::{
    dtos::{
        app_state::AppState,
        response::{GraphqlGenericResponse, GraphqlResponse},
    },
    models::{
        password,
        password_dtos::{
            AddPasswordRequest, DeletePasswordRequest, GetPasswordRequest, GetPasswordsRequest,
            PasswordResponse, PasswordsPageResponse, UpdatePasswordRequest,
        },
        user_dtos::UserRedisSession,
    },
    services::crypto::{decrypt_password, encrypt_password},
    utils::error::{AppError, AppResult},
};

pub async fn add_password(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: AddPasswordRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let database_connection = &app_state.database_connection;
    let user_id = user_redis_session.id;

    let mut existing_password_query =
        password::Entity::find().filter(password::Column::UserId.eq(user_id));

    if let Some(website_url) = &request.website_url {
        existing_password_query =
            existing_password_query.filter(password::Column::WebsiteUrl.eq(website_url));
    } else if let Some(app_name) = &request.app_name {
        existing_password_query =
            existing_password_query.filter(password::Column::AppName.eq(app_name));
    }

    let existing_password = existing_password_query
        .one(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to find password: {}", e.to_string())))?;

    if let Some(_) = existing_password {
        return Err(AppError::Conflict(
            "Password already exists for this website/app".to_string(),
        ));
    }

    let dek = user_redis_session.dek.clone();

    let dek_u8_32: [u8; 32] = dek
        .try_into()
        .map_err(|_| AppError::Crypto("Unable to convert DEK to [u8; 32]".to_string()))?;

    let encrypted_password = encrypt_password(&request.password, &dek_u8_32)?;
    let encrypted_email = request
        .email
        .as_ref()
        .map(|e| encrypt_password(e, &dek_u8_32).unwrap());

    let encrypted_username = request
        .username
        .as_ref()
        .map(|u| encrypt_password(u, &dek_u8_32).unwrap());

    password::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        website_url: Set(request.website_url),
        app_name: Set(request.app_name),
        encrypted_email: Set(encrypted_email),
        encrypted_username: Set(encrypted_username),
        encrypted_password: Set(encrypted_password),
        user_id: Set(user_id),
        ..Default::default()
    }
    .insert(database_connection.as_ref())
    .await
    .map_err(|e| AppError::Internal(format!("Failed to save password: {}", e.to_string())))?;

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Password added successfully".to_string(),
    })
}

pub async fn get_password(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: GetPasswordRequest,
) -> AppResult<GraphqlResponse<PasswordResponse>> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let database_connection = &app_state.database_connection;
    let user_id = user_redis_session.id;

    let dek_u8_32: [u8; 32] = user_redis_session
        .dek
        .clone()
        .try_into()
        .map_err(|_| AppError::Crypto("Unable to convert DEK to [u8; 32]".to_string()))?;

    let password_entry = password::Entity::find()
        .filter(password::Column::UserId.eq(user_id))
        .filter(password::Column::Id.eq(request.id))
        .one(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to find password: {}", e.to_string())))?
        .ok_or(AppError::NotFound("Password not found".to_string()))?;

    let encrypted_password = password_entry.encrypted_password;
    let password = decrypt_password(&encrypted_password, &dek_u8_32)?;
    let email = password_entry
        .encrypted_email
        .map(|e| decrypt_password(&e, &dek_u8_32).unwrap());
    let username = password_entry
        .encrypted_username
        .map(|u| decrypt_password(&u, &dek_u8_32).unwrap());

    Ok(GraphqlResponse::<PasswordResponse> {
        success: true,
        message: "Password found".to_string(),
        data: PasswordResponse {
            id: password_entry.id,
            website_url: password_entry.website_url.clone(),
            app_name: password_entry.app_name.clone(),
            email,
            username,
            password,
            created_at: password_entry.created_at,
            updated_at: password_entry.updated_at,
        },
    })
}

pub async fn update_password(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: UpdatePasswordRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let database_connection = &app_state.database_connection;
    let user_id = user_redis_session.id;
    let dek = user_redis_session.dek.clone();

    let password_entry = password::Entity::find()
        .filter(password::Column::UserId.eq(user_id))
        .filter(password::Column::Id.eq(request.id))
        .one(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to find password: {}", e.to_string())))?
        .ok_or(AppError::NotFound("Password not found".to_string()))?;

    let dek_u8_32: [u8; 32] = dek
        .try_into()
        .map_err(|_| AppError::Crypto("Unable to convert DEK to [u8; 32]".to_string()))?;

    let encrypted_password = encrypt_password(&request.password, &dek_u8_32)?;

    let mut updated_password: password::ActiveModel = password_entry.into();

    updated_password.encrypted_password = Set(encrypted_password);

    let website_url = request.website_url;
    let app_name = request.app_name;
    let username = request.username;
    let email = request.email;

    if let Some(website_url) = &website_url {
        updated_password.website_url = Set(Some(website_url.to_string()));
    }

    if let Some(app_name) = &app_name {
        updated_password.app_name = Set(Some(app_name.to_string()));
    }

    if let Some(username) = &username {
        let encrypted_username = encrypt_password(username, &dek_u8_32)?;
        updated_password.encrypted_username = Set(Some(encrypted_username));
    }

    if let Some(email) = &email {
        let encrypted_email = encrypt_password(email, &dek_u8_32)?;
        updated_password.encrypted_email = Set(Some(encrypted_email));
    }
    updated_password.updated_at = Set(Utc::now());

    updated_password
        .update(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update password: {}", e.to_string())))?;

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Password updated successfully".to_string(),
    })
}

pub async fn delete_password(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: DeletePasswordRequest,
) -> AppResult<GraphqlGenericResponse> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let database_connection = &app_state.database_connection;
    let user_id = user_redis_session.id;

    password::Entity::delete_many()
        .filter(password::Column::UserId.eq(user_id))
        .filter(password::Column::Id.eq(request.id))
        .exec(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete password: {}", e.to_string())))?;

    Ok(GraphqlGenericResponse {
        success: true,
        message: "Password deleted successfully".to_string(),
    })
}

pub async fn get_passwords(
    ctx: &Context<'_>,
    user_redis_session: &UserRedisSession,
    request: GetPasswordsRequest,
) -> AppResult<GraphqlResponse<PasswordsPageResponse>> {
    let app_state = ctx
        .data::<Arc<AppState>>()
        .map_err(|_| AppError::Internal("App State is not passed".to_string()))?;

    let database_connection = &app_state.database_connection;
    let user_id = user_redis_session.id;

    let page = request.page;
    const PAGE_SIZE: u64 = 10;
    let next_page_token = request.next_page_token;

    let dek_u8_32: [u8; 32] = user_redis_session
        .dek
        .clone()
        .try_into()
        .map_err(|_| AppError::Crypto("Unable to convert DEK to [u8; 32]".to_string()))?;

    let mut passwords_select =
        password::Entity::find().filter(password::Column::UserId.eq(user_id));

    if next_page_token.is_some() {
        let updated_at_str = decrypt_password(next_page_token.unwrap().as_str(), &dek_u8_32)?;
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|_| AppError::Crypto("Unable to parse updated_at".to_string()))?;

        passwords_select = passwords_select.filter(password::Column::UpdatedAt.gt(updated_at));
    }

    let passwords = passwords_select
        .order_by(password::Column::CreatedAt, Order::Desc)
        .limit(PAGE_SIZE)
        .all(database_connection.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get passwords: {}", e.to_string())))?;

    if passwords.is_empty() {
        return Ok(GraphqlResponse::<PasswordsPageResponse> {
            success: true,
            message: "No passwords found".to_string(),
            data: PasswordsPageResponse {
                passwords: vec![],
                page,
                next_page_token: None,
            },
        });
    }

    let mut passwords_response: Vec<PasswordResponse> = vec![];

    for password_entry in &passwords {
        let encrypted_password = &password_entry.encrypted_password;
        let password = decrypt_password(&encrypted_password, &dek_u8_32)?;
        let email = password_entry
            .encrypted_email
            .as_ref()
            .map(|e| decrypt_password(&e, &dek_u8_32).unwrap());
        let username = password_entry
            .encrypted_username
            .as_ref()
            .map(|u| decrypt_password(&u, &dek_u8_32).unwrap());

        passwords_response.push(PasswordResponse {
            id: password_entry.id,
            website_url: password_entry.website_url.clone(),
            app_name: password_entry.app_name.clone(),
            email,
            username,
            password,
            created_at: password_entry.created_at,
            updated_at: password_entry.updated_at,
        });
    }

    let next_page_token = passwords
        .last()
        .map(|p| encrypt_password(&p.updated_at.to_rfc3339(), &dek_u8_32).unwrap());

    Ok(GraphqlResponse::<PasswordsPageResponse> {
        success: true,
        message: "Passwords found".to_string(),
        data: PasswordsPageResponse {
            passwords: passwords_response,
            page,
            next_page_token,
        },
    })
}
