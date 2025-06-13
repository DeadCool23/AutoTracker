use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use crate::paths::ROUTE_GET_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::PointData;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteRequest {
    #[schema(example = "example@example.com")]
    user_login: String,
    #[schema(example = "А777МР77")]
    gos_num: String,
    #[schema(example = "01.01.2025")]
    date: String,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteResponse {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
    pub route: Option<Vec<PointData>>,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/car/route",
    summary = "Получение маршрута",
    description = "Получение маршрута по гос.номеру и дате",
    request_body = RouteRequest,
    responses(
        (status = StatusCode::OK, description = "Маршрут успешно получен", body = RouteResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["route"]
)]
pub async fn handle_route(
    ExtractJson(payload): ExtractJson<RouteRequest>,
) -> Result<Json<RouteResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("route_getter").await {
        Some(BLServices::RouteGetService(s)) => s,
        _ => {
            log::warn!("Can't get RouteGetService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let route = match service
        .get_car_route(&payload.gos_num, &payload.user_login, &payload.date)
        .await
    {
        Ok(route) => route,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                let response = RouteResponse {
                    status,
                    route: None,
                };
                log::warn!("Sended error response {:#?}", response);
                return Ok(Json(response));
            }
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("Not founded {e}");
                let response = RouteResponse {
                    status,
                    route: None,
                };
                log::warn!("Sended error response {:#?}", response);
                return Ok(Json(response));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = RouteResponse { status, route };
    log::info!("Sended response {:#?}", response);

    Ok(Json(response))
}
