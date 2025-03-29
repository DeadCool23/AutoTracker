use axum::{routing::put, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    response_status_code::ResponseStatusCode, snap_send_service::handle_snap_send,
    ResponseWithoutData,
};
use api::paths;

#[tokio::test]
async fn test_handle_snap_send_success() {
    let app = Router::new().route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send));
    let server = TestServer::new(app).unwrap();

    let response = server
        .put(&paths::SNAP_SEND_SERVICE_PATH)
        .json(&json!({
            "camera": {
                "id": 1,
                "location": {
                    "latitude": 53.9333,
                    "longitude": 53.9222
                }
            },
            "date": "01.01.2025",
            "gos_num": "А777МР77",
            "time": "8:10"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_snap_send_invalid_gos_num() {
    let app = axum::Router::new().route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send));
    let server = TestServer::new(app).unwrap();

    let response = server
        .put(&paths::SNAP_SEND_SERVICE_PATH)
        .json(&json!({
            "camera": {
                "id": 1,
                "location": {
                    "latitude": 53.9333,
                    "longitude": 53.9222
                }
            },
            "date": "01.01.2025",
            "gos_num": "А777М77",
            "time": "8:10"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_GOS_NUM as isize
    );
}

#[tokio::test]
async fn test_handle_snap_send_invalid_date() {
    let app = axum::Router::new().route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send));
    let server = TestServer::new(app).unwrap();

    let response = server
        .put(&paths::SNAP_SEND_SERVICE_PATH)
        .json(&json!({
            "camera": {
                "id": 1,
                "location": {
                    "latitude": 53.9333,
                    "longitude": 53.9222
                }
            },
            "date": "01-01.2025",
            "gos_num": "А777МР77",
            "time": "8:10"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_DATE as isize);
}

#[tokio::test]
async fn test_handle_snap_send_invalid_time_format() {
    let app = axum::Router::new().route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send));
    let server = TestServer::new(app).unwrap();

    let response = server
        .put(&paths::SNAP_SEND_SERVICE_PATH)
        .json(&json!({
            "camera": {
                "id": 1,
                "location": {
                    "latitude": 53.9333,
                    "longitude": 53.9222
                }
            },
            "date": "01.01.2025",
            "gos_num": "А777МР77",
            "time": "8-10"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_TIME as isize);
}

#[tokio::test]
async fn test_handle_snap_send_time_value() {
    let app = axum::Router::new().route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send));
    let server = TestServer::new(app).unwrap();

    let response = server
        .put(&paths::SNAP_SEND_SERVICE_PATH)
        .json(&json!({
            "camera": {
                "id": 1,
                "location": {
                    "latitude": 53.9333,
                    "longitude": 53.9222
                }
            },
            "date": "01.01.2025",
            "gos_num": "А777МР77",
            "time": "25:60"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_TIME as isize);
}
