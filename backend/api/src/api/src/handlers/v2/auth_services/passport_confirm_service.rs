use super::super::token_auth::get_auth_data;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use crate::paths::{vpath, PASSPORT_CONF_SERVICE_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Json as ExtractJson, Path},
    http::{HeaderMap, StatusCode},
    Json,
};
use models::Document;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[axum::debug_handler]
#[utoipa::path(
    patch,
    path = "/api/v2/users/{id}/passport",
    summary = "Подтверждение паспортных данных",
    description = "Подтверждение паспортных данных пользователя",
    request_body = Document,
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::NO_CONTENT, description = "Пользователь успешно обновил паспортные данные"),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::NOT_FOUND, description = "Пользователь не найден", body = StatusResponse),
        (status = StatusCode::CONFLICT, description = "Паспортные данные уже привязаны к другому пользователю", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["user", "auth"]
)]
pub async fn handle_passport_conf_v2(
    headers: HeaderMap,
    Path(id): Path<u64>,
    ExtractJson(payload): ExtractJson<Document>,
) -> Response {
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    if claim.id != id as usize {
        log::error!("Not enough rights: claim.id={} != path.id={}", claim.id, id);
        return StatusCode::FORBIDDEN.into_response();
    }

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::error!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Err(e) = service.passport_confirm_by_id(id as usize, &payload).await {
        return handle_passport_conf_error(e);
    }

    log::info!("Passport confirmation for user {id} succeeded");
    StatusCode::NO_CONTENT.into_response()
}

fn handle_passport_conf_error(err: ServiceError) -> Response {
    let mut status = StatusResponse::new();

    match err {
        ServiceError::InvalidDataError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
            status.message = format!("Invalid {e}");
            log::warn!("Invalid data: {e}");
            (StatusCode::BAD_REQUEST, Json(status)).into_response()
        }
        ServiceError::IsExistError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::EXIST_DATA) as isize;
            status.message = format!("{e} already exists");
            log::warn!("Conflict: {e}");
            (StatusCode::CONFLICT, Json(status)).into_response()
        }
        ServiceError::NotFoundError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
            status.message = format!("{e} not found");
            log::warn!("Not found: {e}");
            (StatusCode::NOT_FOUND, Json(status)).into_response()
        }
        _ => {
            log::error!("Internal server error during passport confirmation: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
