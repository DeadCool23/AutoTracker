
use super::{ResponseStatusCode, ResponseStatusCodeType};
use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use super::{SearchByGosNumRequest, TrackInfoSearcherResponse};
use crate::paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH as PATH;

use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/track-info/search/by-gos-num-mask",
    request_body = SearchByGosNumRequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_gos_num_mask(
    ExtractJson(payload): ExtractJson<SearchByGosNumRequest>,
) -> Result<Json<TrackInfoSearcherResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("searcher") {
        Some(BLServices::SearchService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.search_track_info_by_gos_num_mask(&payload.gos_num) {
        Ok(track_info) => return Ok(Json(TrackInfoSearcherResponse { status, track_info })),
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code = ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(TrackInfoSearcherResponse { status, track_info: vec![] }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };
}
