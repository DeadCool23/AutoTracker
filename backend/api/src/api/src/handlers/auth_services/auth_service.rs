use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{ResponseStatusCode, StatusResponse};
use crate::paths::AUTH_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct AuthRequest {
    #[schema(example = "email@example.com")]
    email: String,
    #[schema(example = "password")]
    pswd: String,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
    #[schema(value_type=User)]
    pub user: Option<User>,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/user/auth",
    request_body = AuthRequest,
    summary = "Аутентификация",
    description = "Аутентификация пользователя по логину и паролю",
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно авторизирован", body = AuthResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["auth"]
)]
pub async fn handle_auth(
    ExtractJson(payload): ExtractJson<AuthRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("auther").await {
        Some(BLServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user = match service.auth(&payload.email, &payload.pswd).await {
        Ok(user) => user,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code = ResponseStatusCode::INVALID_AUTH_DATA as isize;
                status.message = format!("Invalid {e}");
                let response = AuthResponse { status, user: None };
                log::warn!("Sended error response {:#?}", response);

                return Ok(Json(response));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = AuthResponse {
        status,
        user: Some(user),
    };
    log::info!("Sended response {:#?}", response);

    Ok(Json(response))
}
