
use super::{SearchByFIORequest, TrackInfoSearcherResponse};
use super::{BLServices, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use crate::paths::TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH as PATH;

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/track-info/search/by-fio",
    request_body = SearchByFIORequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_fio(
    ExtractJson(payload): ExtractJson<SearchByFIORequest>,
) -> Result<Json<TrackInfoSearcherResponse>, StatusCode> {
    let status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("searcher") {
        Some(BLServices::SearchService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.search_track_info_by_owner_fio(&payload.name, &payload.surname, &payload.lastname) {
        Ok(track_info) => return Ok(Json(TrackInfoSearcherResponse { status, track_info })),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };
}
