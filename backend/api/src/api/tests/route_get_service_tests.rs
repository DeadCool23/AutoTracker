use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    response_status_code::ResponseStatusCode,
    route_get_service::{handle_route, RouteResponse},
};
use api::paths;

#[tokio::test]
async fn test_handle_route_success() {
    let app = Router::new().route(&paths::ROUTE_GET_SERVICE_PATH, post(handle_route));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::ROUTE_GET_SERVICE_PATH)
        .json(&json!({
            "user_id": 1,
            "gos_num": "А777МР77",
            "date": "01.01.2025"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: RouteResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
    assert!(body.route.is_some());
}

#[tokio::test]
async fn test_handle_route_invalid_date() {
    let app = axum::Router::new().route(&paths::ROUTE_GET_SERVICE_PATH, post(handle_route));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::ROUTE_GET_SERVICE_PATH)
        .json(&json!({
            "user_id": 1,
            "gos_num": "А777МР77",
            "date": "invalid-date"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: RouteResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_DATE as isize);
}

#[tokio::test]
async fn test_handle_route_invalid_gos_num() {
    let app = axum::Router::new().route(&paths::ROUTE_GET_SERVICE_PATH, post(handle_route));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::ROUTE_GET_SERVICE_PATH)
        .json(&json!({
            "user_id": 1,
            "gos_num": "А7777МР78878",
            "date": "01.01.2023"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: RouteResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_GOS_NUM as isize
    );
}
