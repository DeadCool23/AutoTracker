use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, StatusResponse};
use crate::paths::{vpath, AUTH_SERVICE_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct AuthRequest {
    #[schema(example = "email@example.com")]
    pub email: String,
    #[schema(example = "password")]
    pub pswd: String,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    #[schema(example = "jwt-example")]
    pub jwt: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/v2/users/login",
    request_body = AuthRequest,
    summary = "Аутентификация",
    description = "Аутентификация пользователя по логину и паролю",
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно авторизирован", body = String),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["auth"]
)]
pub async fn handle_auth_v2(ExtractJson(payload): ExtractJson<AuthRequest>) -> Response {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let user = match service.auth_with_id(&payload.email, &payload.pswd).await {
        Ok(user) => user,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code = ResponseStatusCode::INVALID_AUTH_DATA as isize;
                status.message = format!("Invalid {e}");
                log::warn!("Sended error response {:#?}", status);

                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };

    let response = match jwt_processing::create_jwt(&user) {
        Ok(token) => token,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    log::info!("Sended response {:#?}", response);

    response.into_response()
}
