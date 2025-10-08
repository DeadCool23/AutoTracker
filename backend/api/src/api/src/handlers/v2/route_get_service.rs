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
use jwt_processing::Claims;
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

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteResponse {
    pub route: Vec<PointData>,
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
        (status = StatusCode::OK, description = "Маршрут успешно получен", body = RouteResponse),
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
        Some(CoreServices::RouteGetService(s)) => CoreServices::RouteGetService(s),
        _ => {
            log::error!("Can't get RouteGetService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Err(resp) = verify_user_access(&claim, &payload.gos_num, &service).await {
        return resp;
    }

    match get_car_route_with_response(&service, &claim, &payload).await {
        Ok(route) => {
            log::info!("Sended response {:#?}", route);
            Json(RouteResponse { route }).into_response()
        }
        Err(resp) => resp,
    }
}

fn build_error_response(
    err_msg: &str,
    code_type: ResponseStatusCodeType,
    status: StatusCode,
) -> Response {
    let mut status_body = StatusResponse::new();
    status_body.code = ResponseStatusCode::from(err_msg, code_type) as isize;
    status_body.message = format!("{err_msg}");

    log::warn!("Sending error response {:#?}", status_body);
    (status, Json(status_body)).into_response()
}

async fn verify_user_access(
    claim: &Claims,
    gos_num: &str,
    service: &CoreServices,
) -> Result<(), Response> {
    let serv = match service {
        CoreServices::RouteGetService(s) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    match claim.role {
        Role::audit => Ok(()),
        Role::user => {
            match serv.is_car_owned_by_user(claim.id, &gos_num.to_string()).await {
                Ok(true) => Ok(()),
                Ok(false) => {
                    log::warn!("User {} not owner of car {}", claim.id, gos_num);
                    Err(StatusCode::FORBIDDEN.into_response())
                }
                Err(ServiceError::InvalidDataError(e)) => {
                    let resp = build_error_response(
                        &e,
                        ResponseStatusCodeType::INVALID_DATA,
                        StatusCode::BAD_REQUEST,
                    );
                    Err(resp)
                }
                Err(e) => {
                    log::error!("Can't check ownership: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                }
            }
        }
        _ => Err(StatusCode::FORBIDDEN.into_response()),
    }
}

async fn get_car_route_with_response(
    service: &CoreServices,
    claim: &Claims,
    payload: &RouteRequest,
) -> Result<Vec<PointData>, Response> {
    let serv = match service {
        CoreServices::RouteGetService(s) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    match serv
        .get_car_route_with_user_id(claim.id, &payload.gos_num, &payload.date)
        .await
    {
        Ok(route) => Ok(route),
        Err(ServiceError::InvalidDataError(e)) => {
            let resp = build_error_response(&e, ResponseStatusCodeType::INVALID_DATA, StatusCode::BAD_REQUEST);
            Err(resp)
        }
        Err(ServiceError::NotFoundError(e)) => {
            let resp = build_error_response(
                &e,
                ResponseStatusCodeType::NOT_FOUNDED_DATA,
                StatusCode::NOT_FOUND,
            );
            Err(resp)
        }
        Err(e) => {
            log::error!("Unhandled route service error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}