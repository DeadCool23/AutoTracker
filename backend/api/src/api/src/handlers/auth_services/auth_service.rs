use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use super::{ResponseStatusCode, StatusResponse};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::paths::AUTH_SERVICE_PATH as PATH;

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
    pub user: Option<User>,
}

#[utoipa::path(
    post,
    path = "/user/auth",
    request_body = AuthRequest,
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
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("auther") {
        Some(BLServices::AuthService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let user = match service.auth(&payload.email, &payload.pswd) {
        Ok(user) => user,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code = ResponseStatusCode::INVALID_AUTH_DATA as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(AuthResponse { status, user: None }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    };

    let response = AuthResponse {
        status,
        user: Some(user),
    };

    Ok(Json(response))
}
