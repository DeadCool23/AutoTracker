use super::CarSearcherResponse;
use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use crate::paths::TRACK_INFO_SEARCH_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::StatusResponse;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchCarByFilterRequest {
    #[schema(example = "name")]
    pub name: Option<String>,
    #[schema(example = "surname")]
    pub surname: Option<String>,
    #[schema(example = "lastname")]
    pub lastname: Option<String>,
    #[schema(value_type = Document)]
    pub passport: Option<Document>,
    #[schema(example = "А*23**99")]
    pub gos_num: Option<String>,
}

#[utoipa::path(
    post,
    path = "/car/search",
    summary = "Поиск автомобилей",
    description = "Поиск автомобилей по общим фильтрам",
    request_body = SearchCarByFilterRequest,
    responses(
        (status = StatusCode::OK, description = "Автомобили успешно найдены", body = CarSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "car"]
)]
pub async fn handle_search_cars_by_filters(
    ExtractJson(payload): ExtractJson<SearchCarByFilterRequest>,
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
        .search_car(
            payload.name,
            payload.surname,
            payload.lastname,
            payload.passport,
            payload.gos_num,
        )
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
