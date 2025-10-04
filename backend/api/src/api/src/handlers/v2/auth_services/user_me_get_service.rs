use super::super::token_auth::get_auth_data;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};
use crate::paths::{vpath, USER_ME_SERVICE_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use models::UserWithId;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/v2/users/me",
    summary = "Получение данных пользователя",
    description = "Получение данных авторизированного пользователя",
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно получен", body = UserWithId),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::NOT_FOUND, description = "Пользователь не найден", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["user"]
)]
pub async fn handle_get_user_me_v2(headers: HeaderMap) -> Response {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}",
        vpath(VERSION, PATH.as_str()),
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let user = match service.get_user_by_id(claim.id).await {
        Ok(user) => user,
        Err(e) => match e {
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("Not founded {e}");
                log::warn!("Sended error response {:#?}", status);

                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };

    log::info!("Sended response {:#?}", user);

    Json(user).into_response()
}
