
use super::{ResponseStatusCode, ResponseStatusCodeType};
use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use super::{SearchByGosNumRequest, CarSearcherResponse};

use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use crate::paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH as PATH;

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/car/search/by-gos-num-mask",
    request_body = SearchByGosNumRequest,
    responses(
        (status = StatusCode::OK, description = "Автомобили успешно найдены", body = CarSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "car"]
)]
pub async fn handle_search_car_by_gos_num_mask(
    ExtractJson(payload): ExtractJson<SearchByGosNumRequest>,
) -> Result<Json<CarSearcherResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);


    let service = match BUSINESS_PROCCESS.get("searcher") {
        Some(BLServices::SearchService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.search_cars_by_gos_num_mask(&payload.gos_num) {
        Ok(cars) => return Ok(Json(CarSearcherResponse { status, cars })),
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code = ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(CarSearcherResponse { status, cars: vec![] }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };
}
