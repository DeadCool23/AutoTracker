use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    search_services::car_search_services::{handle_search_car_by_passport, CarSearcherResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_search_car_by_passport_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH,
        post(handle_search_car_by_passport),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH)
        .json(&json!({
            "passport": {
                "serial": "1111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_car_by_passport_invalid_passport_serial() {
    let app = axum::Router::new().route(
        &paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH,
        post(handle_search_car_by_passport),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH)
        .json(&json!({
            "passport": {
                "serial": "111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_PASSPORT as isize
    );
}

#[tokio::test]
async fn test_handle_search_car_by_passport_invalid_passport_number() {
    let app = axum::Router::new().route(
        &paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH,
        post(handle_search_car_by_passport),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH)
        .json(&json!({
            "passport": {
                "serial": "1111",
                "number": "1111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_PASSPORT as isize
    );
}
