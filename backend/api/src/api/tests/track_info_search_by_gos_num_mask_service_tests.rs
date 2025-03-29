use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    search_services::track_info_search_services::{handle_search_track_info_by_gos_num_mask, TrackInfoSearcherResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_search_track_info_by_gos_num_mask_success() {
    let app = Router::new().route(
        &paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
        post(handle_search_track_info_by_gos_num_mask),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH)
        .json(&json!({
            "gos_num": "А7**М*77"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: TrackInfoSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_track_info_by_gos_num_mask_invalid_gos_num_mask() {
    let app = axum::Router::new().route(
        &paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
        post(handle_search_track_info_by_gos_num_mask),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH)
        .json(&json!({
            "gos_num": "А7**М***"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: TrackInfoSearcherResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_GOS_NUM_MASK as isize
    );
}
