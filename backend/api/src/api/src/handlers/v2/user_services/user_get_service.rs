use super::super::token_auth::get_auth_data;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};
use crate::paths::{vpath, USER_ID_SERVICES_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    Json,
};
use models::UserWithId;

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/v2/users/{id}",
    summary = "Получение данных пользователя",
    description = "Получение данных пользователя по id",
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно получен", body = UserWithId),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::NOT_FOUND, description = "Пользователь не найден", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["user"]
)]
pub async fn handle_get_user_by_id_v2(headers: HeaderMap, Path(id): Path<usize>) -> Response {
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        id
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    if claim.id != id {
        log::warn!("Access denied: claim.id = {}, path.id = {}", claim.id, id);
        return StatusCode::FORBIDDEN.into_response();
    }

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::error!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match service.get_user_by_id(id).await {
        Ok(user) => {
            log::info!("Sending response: {:#?}", user);
            Json(user).into_response()
        }
        Err(e) => handle_get_user_by_id_error(e),
    }
}

fn handle_get_user_by_id_error(err: ServiceError) -> Response {
    let mut status = StatusResponse::new();

    match err {
        ServiceError::NotFoundError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
            status.message = format!("Not found: {e}");
            log::warn!("User not found: {e}");
            (StatusCode::BAD_REQUEST, Json(status)).into_response()
        }
        ServiceError::InvalidDataError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
            status.message = format!("Invalid data: {e}");
            log::warn!("Invalid data: {e}");
            (StatusCode::BAD_REQUEST, Json(status)).into_response()
        }
        _ => {
            log::error!("Unexpected service error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
