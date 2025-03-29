use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    search_services::car_search_services::{handle_search_car_by_fio, CarSearcherResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_search_car_by_fio_full_fio_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_FIO_SERVICE_PATH,
        post(handle_search_car_by_fio),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_FIO_SERVICE_PATH)
        .json(&json!({
            "name": "Ivan",
            "surname": "Ivanov",
            "lastname": "Ivanovich"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_car_by_fio_without_name_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_FIO_SERVICE_PATH,
        post(handle_search_car_by_fio),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_FIO_SERVICE_PATH)
        .json(&json!({
            "name": None::<String>,
            "surname": "Ivanov",
            "lastname": "Ivanovich"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_car_by_fio_without_surname_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_FIO_SERVICE_PATH,
        post(handle_search_car_by_fio),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_FIO_SERVICE_PATH)
        .json(&json!({
            "name": "Ivan",
            "surname": None::<String>,
            "lastname": "Ivanovich"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_search_car_by_fio_without_lastname_success() {
    let app = Router::new().route(
        &paths::CAR_SEARCH_BY_FIO_SERVICE_PATH,
        post(handle_search_car_by_fio),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::CAR_SEARCH_BY_FIO_SERVICE_PATH)
        .json(&json!({
            "name": "Ivan",
            "surname": "Ivanov",
            "lastname": None::<String>
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: CarSearcherResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}