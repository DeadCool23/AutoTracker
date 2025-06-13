use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{CarSearcherResponse, SearchByGosNumRequest};
use super::{ResponseStatusCode, ResponseStatusCodeType};

use crate::paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/car/search/by-gos-num-mask",
    summary = "Поиск автомобиля",
    description = "Поиск автомобиля по маске гос.номера",
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
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("searcher").await {
        Some(BLServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service.search_cars_by_gos_num_mask(&payload.gos_num).await {
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
