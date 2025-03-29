use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    search_services::car_search_services::{handle_search_car_by_gos_num_mask, CarSearcherResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_search_car_by_gos_num_mask_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
        post(handle_search_car_by_gos_num_mask),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH)
        .json(&json!({
            "gos_num": "А7**М*77"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_car_by_gos_num_mask_invalid_gos_num_mask() {
    let app = axum::Router::new().route(
        &paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
        post(handle_search_car_by_gos_num_mask),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH)
        .json(&json!({
            "gos_num": "А7**М***"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_GOS_NUM_MASK as isize
    );
}
