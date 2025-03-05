use validator::ValidationError;

use crate::models::password_dtos::{
    AddPasswordRequest, GetPasswordRequest, GetPasswordsRequest, UpdatePasswordRequest,
};

pub fn validate_add_password_request<'v_a>(
    add_password_request: &'v_a AddPasswordRequest,
) -> Result<(), ValidationError> {
    if add_password_request.website_url.is_none() && add_password_request.app_name.is_none() {
        return Err(ValidationError::new(
            "Either website_url or app_name is required",
        ));
    }

    if add_password_request.username.is_none() && add_password_request.email.is_none() {
        return Err(ValidationError::new("Either username or email is required"));
    }

    Ok(())
}

pub fn validate_update_password_request<'v_a>(
    add_password_request: &'v_a UpdatePasswordRequest,
) -> Result<(), ValidationError> {
    if add_password_request.website_url.is_none() && add_password_request.app_name.is_none() {
        return Err(ValidationError::new(
            "Either website_url or app_name is required",
        ));
    }

    if add_password_request.username.is_none() && add_password_request.email.is_none() {
        return Err(ValidationError::new("Either username or email is required"));
    }

    Ok(())
}

pub fn validate_get_passwords_request<'v_a>(
    get_passwords_request: &'v_a GetPasswordsRequest,
) -> Result<(), ValidationError> {
    if get_passwords_request.page <= 0 {
        return Err(ValidationError::new("Page must be greater than 0"));
    }

    if get_passwords_request.page == 1 && get_passwords_request.next_page_token.is_some() {
        return Err(ValidationError::new(
            "Next page token is not required for page 1",
        ));
    }

    if get_passwords_request.page > 1 && get_passwords_request.next_page_token.is_none() {
        return Err(ValidationError::new(
            "Next page token is required for page greater than 1",
        ));
    }

    Ok(())
}

pub fn validate_get_password_request<'v_a>(
    get_passwords_request: &'v_a GetPasswordRequest,
) -> Result<(), ValidationError> {
    if get_passwords_request.id.is_none() {
        if get_passwords_request.website_url.is_none() && get_passwords_request.app_name.is_none() {
            return Err(ValidationError::new(
                "Id or website_url or app_name is required",
            ));
        }

        if get_passwords_request.username.is_none() && get_passwords_request.email.is_none() {
            return Err(ValidationError::new("Either username or email is required"));
        }
    }

    Ok(())
}
