use super::token_auth::get_auth_data;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use crate::paths::{vpath, ROUTE_GET_SERVICE_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Json as ExtractJson,
    http::{HeaderMap, StatusCode},
    Json,
};
use models::{PointData, Role};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteRequest {
    #[schema(example = "А777МР77")]
    pub gos_num: String,
    #[schema(example = "01.01.2025")]
    pub date: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/v2/cars/route",
    summary = "Получение маршрута",
    description = "Получение маршрута по гос.номеру и дате",
    request_body = RouteRequest,
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::OK, description = "Маршрут успешно получен", body = Vec<PointData>),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::NOT_FOUND, description = "Данные не найдены", body = StatusResponse),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["route", "car"]
)]
pub async fn handle_route_v2(
    headers: HeaderMap,
    ExtractJson(payload): ExtractJson<RouteRequest>,
) -> Response {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    let service = match ServicesContainer::get("route_getter").await {
        Some(CoreServices::RouteGetService(s)) => s,
        _ => {
            log::warn!("Can't get RouteGetService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if claim.role != Role::audit {
        if claim.role == Role::user {
            let is_owner = match service
                .is_car_owned_by_user(claim.id, &payload.gos_num)
                .await
            {
                Ok(check) => check,
                Err(e) => match e {
                    ServiceError::InvalidDataError(e) => {
                        status.code =
                            ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA)
                                as isize;
                        status.message = format!("Invalid {e}");

                        log::warn!("Sended error response {:#?}", status);
                        return (StatusCode::BAD_REQUEST, Json(status)).into_response();
                    }
                    _ => {
                        log::error!("Can't check ownership: {:?}", e);
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                },
            };

            if !is_owner {
                log::error!("User {} not owner of car {}", claim.id, &payload.gos_num);
                return StatusCode::FORBIDDEN.into_response();
            }
        } else {
            return StatusCode::FORBIDDEN.into_response();
        }
    }

    let route = match service
        .get_car_route_with_user_id(claim.id, &payload.gos_num, &payload.date)
        .await
    {
        Ok(route) => route,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");

                log::warn!("Sended error response {:#?}", status);
                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("Not founded {e}");

                log::warn!("Sended error response {:#?}", status);
                return (StatusCode::NOT_FOUND, Json(status)).into_response();
            }
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };

    log::info!("Sended response {:#?}", route);

    Json(route).into_response()
}
