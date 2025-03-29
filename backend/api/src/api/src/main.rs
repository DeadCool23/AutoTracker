mod handlers;
mod paths;

use axum::{
    routing::{post, put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use handlers::{
    auth_services::{
        auth_service::handle_auth, passport_confirm_service::handle_passport_conf,
        registration_service::handle_reg,
    },
    route_get_service::handle_route,
    snap_send_service::handle_snap_send,
    search_services::{
        car_search_services::{
            handle_search_car_by_fio,
            handle_search_car_by_passport,
            handle_search_car_by_gos_num_mask
        },
        track_info_search_services::{
            handle_search_track_info_by_fio,
            handle_search_track_info_by_date,
            handle_search_track_info_by_passport,
            handle_search_track_info_by_gos_num_mask
        }
    },
    ApiDoc,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .route(&paths::ROUTE_GET_SERVICE_PATH, post(handle_route))
        .route(&paths::AUTH_SERVICE_PATH, post(handle_auth))
        .route(&paths::REG_SERVICE_PATH, post(handle_reg))
        .route(
            &paths::PASSPORT_CONF_SERVICE_PATH,
            post(handle_passport_conf),
        )
        .route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send))
        .route(&paths::CAR_SEARCH_BY_FIO_SERVICE_PATH, post(handle_search_car_by_fio))
        .route(&paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH, post(handle_search_car_by_passport))
        .route(&paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH, post(handle_search_car_by_gos_num_mask))
        .route(&paths::TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH, post(handle_search_track_info_by_fio))
        .route(&paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH, post(handle_search_track_info_by_date))
        .route(&paths::TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH, post(handle_search_track_info_by_passport))
        .route(&paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH, post(handle_search_track_info_by_gos_num_mask))
        .merge(
            SwaggerUi::new(paths::DOCS_PATH.as_str())
                .url(paths::OPENAPI_DOCS_PATH.as_str(), ApiDoc::openapi()),
        );

    let addr = dotenv::var("API_URL")
        .ok()
        .unwrap_or("127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
