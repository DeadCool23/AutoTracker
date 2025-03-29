
use super::{SearchByFIORequest, CarSearcherResponse};

use super::{BLServices, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use crate::paths::CAR_SEARCH_BY_FIO_SERVICE_PATH as PATH;

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/car/search/by-fio",
    request_body = SearchByFIORequest,
    responses(
        (status = StatusCode::OK, description = "Автомобили успешно найдены", body = CarSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "car"]
)]
pub async fn handle_search_car_by_fio(
    ExtractJson(payload): ExtractJson<SearchByFIORequest>,
) -> Result<Json<CarSearcherResponse>, StatusCode> {
    let status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);


    let service = match BUSINESS_PROCCESS.get("searcher") {
        Some(BLServices::SearchService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.search_cars_by_owner_fio(&payload.name, &payload.surname, &payload.lastname) {
        Ok(cars) => return Ok(Json(CarSearcherResponse { status, cars })),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
}
