use super::{CarSearcherResponse, SearchByFIORequest};

use super::{BLServices, BUSINESS_SERVICES};
use crate::paths::CAR_SEARCH_BY_FIO_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/car/search/by-fio",
    summary = "Поиск автомобиля",
    description = "Поиск автомобиля по ФИО владельца",
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
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("searcher").await {
        Some(BLServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .search_cars_by_owner_fio(payload.name, payload.surname, payload.lastname)
        .await
    {
        Ok(cars) => CarSearcherResponse { status, cars },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
