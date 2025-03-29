use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Location;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::paths::ROUTE_GET_SERVICE_PATH as PATH;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteRequest {
    #[schema(example = 1)]
    user_id: usize,
    #[schema(example = "А777МР77")]
    gos_num: String,
    #[schema(example = "01.01.2025")]
    date: String,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RouteResponse {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
    #[schema(example = json!([
        { "longitude": 54.98989 , "latitude": 56.89882 }]
    ))]
    pub route: Option<Vec<Location>>,
}

#[utoipa::path(
    post,
    path = "/car/route",
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
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("route_getter") {
        Some(BLServices::RouteGetService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let route = match service.get_car_route(&payload.gos_num, payload.user_id, &payload.date) {
        Ok(route) => route,
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(RouteResponse {
                    status,
                    route: None,
                }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = RouteResponse { status, route };

    Ok(Json(response))
}
