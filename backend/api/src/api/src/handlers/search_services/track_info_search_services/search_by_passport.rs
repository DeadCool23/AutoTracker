use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use super::{SearchByPassportRequest, TrackInfoSearcherResponse};

use crate::paths::TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/track-info/search/by-passport",
    summary = "Поиск отслеживаний",
    description = "Поиск отслеживаний по паспортным данным",
    request_body = SearchByPassportRequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_passport(
    ExtractJson(payload): ExtractJson<SearchByPassportRequest>,
) -> Result<Json<TrackInfoSearcherResponse>, StatusCode> {
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
        .search_track_info_by_owner_passport(&payload.passport)
        .await
    {
        Ok(track_info) => TrackInfoSearcherResponse { status, track_info },
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                TrackInfoSearcherResponse {
                    status,
                    track_info: vec![],
                }
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
