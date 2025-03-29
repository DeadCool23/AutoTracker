use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    search_services::track_info_search_services::{handle_search_track_info_by_date, TrackInfoSearcherResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_search_track_info_by_date_success() {
    let app = Router::new().route(
        &paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH,
        post(handle_search_track_info_by_date),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH)
        .json(&json!({
            "date": "01.01.2025"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: TrackInfoSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_track_info_by_date_invalid_date() {
    let app = axum::Router::new().route(
        &paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH,
        post(handle_search_track_info_by_date),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH)
        .json(&json!({
            "date": "01-01-2025"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: TrackInfoSearcherResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_DATE as isize
    );
}
