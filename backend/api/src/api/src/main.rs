mod handlers;
mod paths;

use axum::{
    routing::{get, post, put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use handlers::{
    auth_services::{
        auth_service::handle_auth, passport_confirm_service::handle_passport_conf,
        registration_service::handle_reg,
    },
    camera_service::{
        camera_get_by_cords_service::handle_get_camera_by_cords,
        camera_get_by_id_service::handle_get_camera_by_id,
        get_avg_speed_service::handle_get_avg_speed_for_car_on_camera,
    },
    route_get_service::handle_route,
    search_services::{
        car_search_services::{
            handle_search_car_by_fio, handle_search_car_by_gos_num_mask,
            handle_search_car_by_passport, handle_search_cars_by_filters,
        },
        track_info_search_services::{
            handle_search_track_info_by_date, handle_search_track_info_by_filters,
            handle_search_track_info_by_fio, handle_search_track_info_by_gos_num_mask,
            handle_search_track_info_by_passport,
        },
    },
    snap_send_service::handle_snap_send,
    ApiDoc,
};

#[tokio::main]
async fn main() {
    logger::init(
        &format!("{}/{}", cfg::var("logs.logs_dir"), cfg::var("logs.api_log")),
        true,
    );

    let app = Router::new()
        .route(&paths::ROUTE_GET_SERVICE_PATH, post(handle_route))
        .route(&paths::AUTH_SERVICE_PATH, post(handle_auth))
        .route(&paths::REG_SERVICE_PATH, post(handle_reg))
        .route(
            &paths::PASSPORT_CONF_SERVICE_PATH,
            post(handle_passport_conf),
        )
        .route(&paths::SNAP_SEND_SERVICE_PATH, put(handle_snap_send))
        .route(
            &paths::CAMERA_GET_BY_ID_SERVICE_PATH,
            get(handle_get_camera_by_id),
        )
        .route(
            &paths::CAMERA_GET_BY_CORDS_SERVICE_PATH,
            post(handle_get_camera_by_cords),
        )
        .route(
            &paths::GET_AVG_SPEED_ON_CAMERA_PATH,
            post(handle_get_avg_speed_for_car_on_camera),
        )
        .route(
            &paths::CAR_SEARCH_SERVICE_PATH,
            post(handle_search_cars_by_filters),
        )
        .route(
            &paths::CAR_SEARCH_BY_FIO_SERVICE_PATH,
            post(handle_search_car_by_fio),
        )
        .route(
            &paths::CAR_SEARCH_BY_PASSPORT_SERVICE_PATH,
            post(handle_search_car_by_passport),
        )
        .route(
            &paths::CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
            post(handle_search_car_by_gos_num_mask),
        )
        .route(
            &paths::TRACK_INFO_SEARCH_SERVICE_PATH,
            post(handle_search_track_info_by_filters),
        )
        .route(
            &paths::TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH,
            post(handle_search_track_info_by_fio),
        )
        .route(
            &paths::TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH,
            post(handle_search_track_info_by_date),
        )
        .route(
            &paths::TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH,
            post(handle_search_track_info_by_passport),
        )
        .route(
            &paths::TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH,
            post(handle_search_track_info_by_gos_num_mask),
        )
        .merge(
            SwaggerUi::new(paths::DOCS_PATH.as_str())
                .url(paths::OPENAPI_DOCS_PATH.as_str(), ApiDoc::openapi()),
        );

    let addr = cfg::var("server.api_url");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
