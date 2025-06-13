use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{CarSearcherResponse, SearchByPassportRequest};
use super::{ResponseStatusCode, ResponseStatusCodeType};

use crate::paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/car/search/by-passport",
    summary = "Поиск автомобиля",
    description = "Поиск автомобиля по паспортным данным",
    request_body = SearchByPassportRequest,
    responses(
        (status = StatusCode::OK, description = "Автомобили успешно найдены", body = CarSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "car"]
)]
pub async fn handle_search_car_by_passport(
    ExtractJson(payload): ExtractJson<SearchByPassportRequest>,
) -> Result<Json<CarSearcherResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("searcher").await {
        Some(BLServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .search_cars_by_owner_passport(&payload.passport)
        .await
    {
        Ok(cars) => CarSearcherResponse { status, cars },
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                CarSearcherResponse {
                    status,
                    cars: vec![],
                }
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
