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
    tags = ["auth"]
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
        log::error!("Not enough rights");
        return StatusCode::FORBIDDEN.into_response();
    }

    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: id {} {:?}",
        vpath(VERSION, PATH.as_str()),
        id,
        payload
    );

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match service.passport_confirm_by_id(id as usize, &payload).await {
        Ok(_) => {}
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");

                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            ServiceError::IsExistError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::EXIST_DATA) as isize;
                status.message = format!("{e} is exist");

                return (StatusCode::CONFLICT, Json(status)).into_response();
            }
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("{e} not founded");

                return (StatusCode::NOT_FOUND, Json(status)).into_response();
            }
            _ => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        },
    };

    log::info!("Request status {:#?}", status);
    (StatusCode::NO_CONTENT).into_response()
}
