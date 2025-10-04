use super::super::token_auth::get_auth_data;
use super::StatusResponse;
use super::VERSION;
use super::{CoreServices, ServicesContainer};
use crate::paths::{vpath, USER_ID_SERVICES_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
};

#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/api/v2/users/{id}",
    summary = "Удаление пользователя",
    description = "Удаление пользователя по id",
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Пользователь успешно удален"),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::NOT_FOUND, description = "Пользователь не найден", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["user"]
)]
pub async fn handle_delete_user_by_id_v2(headers: HeaderMap, Path(id): Path<usize>) -> Response {
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        id
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    if claim.id != id as usize {
        log::warn!("Not enough rights");
        return StatusCode::FORBIDDEN.into_response();
    }

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match service.delete_user_by_id(id).await {
        Ok(_) => {
            log::info!("User successfully deleted");
            return StatusCode::NO_CONTENT.into_response();
        }
        Err(_) => {
            log::error!("User can't be deleted");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
}
