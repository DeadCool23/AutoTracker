use crate::paths::*;
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::v1::{
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
    ApiDoc as ApiDocV1,
};

fn v1_path(path: &str) -> String {
    vpath(1, path)
}

pub fn init_v1() -> Router {
    Router::new()
        .route(
            &v1_path(ROUTE_GET_SERVICE_PATH.as_str()),
            post(handle_route),
        )
        .route(&v1_path(AUTH_SERVICE_PATH.as_str()), post(handle_auth))
        .route(&REG_SERVICE_PATH, post(handle_reg))
        .route(
            &v1_path(PASSPORT_CONF_SERVICE_PATH.as_str()),
            post(handle_passport_conf),
        )
        .route(
            &v1_path(SNAP_SEND_SERVICE_PATH.as_str()),
            put(handle_snap_send),
        )
        .route(
            &v1_path(CAMERA_GET_BY_ID_SERVICE_PATH.as_str()),
            get(handle_get_camera_by_id),
        )
        .route(
            &v1_path(CAMERA_GET_BY_CORDS_SERVICE_PATH.as_str()),
            post(handle_get_camera_by_cords),
        )
        .route(
            &v1_path(GET_AVG_SPEED_ON_CAMERA_PATH.as_str()),
            post(handle_get_avg_speed_for_car_on_camera),
        )
        .route(
            &v1_path(CAR_SEARCH_SERVICE_PATH.as_str()),
            post(handle_search_cars_by_filters),
        )
        .route(
            &v1_path(CAR_SEARCH_BY_FIO_SERVICE_PATH.as_str()),
            post(handle_search_car_by_fio),
        )
        .route(
            &v1_path(CAR_SEARCH_BY_PASSPORT_SERVICE_PATH.as_str()),
            post(handle_search_car_by_passport),
        )
        .route(
            &v1_path(CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH.as_str()),
            post(handle_search_car_by_gos_num_mask),
        )
        .route(
            &v1_path(TRACK_INFO_SEARCH_SERVICE_PATH.as_str()),
            post(handle_search_track_info_by_filters),
        )
        .route(
            &v1_path(TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH.as_str()),
            post(handle_search_track_info_by_fio),
        )
        .route(
            &v1_path(TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH.as_str()),
            post(handle_search_track_info_by_date),
        )
        .route(
            &v1_path(TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH.as_str()),
            post(handle_search_track_info_by_passport),
        )
        .route(
            &v1_path(TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH.as_str()),
            post(handle_search_track_info_by_gos_num_mask),
        )
        .merge(
            SwaggerUi::new(v1_path(DOCS_PATH.as_str()))
                .url(v1_path(OPENAPI_DOCS_PATH.as_str()), ApiDocV1::openapi()),
        )
}

use crate::handlers::v2::{
    auth_services::{
        auth_service::handle_auth_v2, passport_confirm_service::handle_passport_conf_v2,
        registration_service::handle_reg_v2,
    },
    route_get_service::handle_route_v2,
    search_services::{
        car_search_service::handle_search_cars_by_filters_with_offset_v2,
        track_info_search_service::handle_search_track_info_by_filters_with_offset_v2,
    },
    user_services::{
        user_delete_service::handle_delete_user_by_id_v2,
        user_get_service::handle_get_user_by_id_v2, user_me_get_service::handle_get_user_me_v2,
    },
    ApiDoc as ApiDocV2,
};

#[allow(dead_code)]
fn v2_path(path: &str) -> String {
    vpath(2, path)
}

pub fn init_v2() -> Router {
    Router::new()
        .route(
            &v2_path(AUTH_SERVICE_V2_PATH.as_str()),
            post(handle_auth_v2),
        )
        .route(
            &v2_path(USER_ME_SERVICE_V2_PATH.as_str()),
            get(handle_get_user_me_v2),
        )
        .route(
            &v2_path(USER_ID_SERVICES_V2_PATH.as_str()),
            get(handle_get_user_by_id_v2),
        )
        .route(
            &v2_path(USER_ID_SERVICES_V2_PATH.as_str()),
            delete(handle_delete_user_by_id_v2),
        )
        .route(
            &v2_path(PASSPORT_CONF_SERVICE_V2_PATH.as_str()),
            patch(handle_passport_conf_v2),
        )
        .route(&v2_path(REG_SERVICE_V2_PATH.as_str()), post(handle_reg_v2))
        .route(
            &v2_path(CARS_SEARCH_SERVICE_PATH_V2.as_str()),
            post(handle_search_cars_by_filters_with_offset_v2),
        )
        .route(
            &v2_path(TRACK_INFOS_SEARCH_SERVICE_PATH_V2.as_str()),
            post(handle_search_track_info_by_filters_with_offset_v2),
        )
        .route(
            &v2_path(ROUTE_GET_SERVICE_PATH_V2.as_str()),
            post(handle_route_v2),
        )
        .merge(
            SwaggerUi::new(v2_path(DOCS_PATH.as_str()))
                .url(v2_path(OPENAPI_DOCS_PATH.as_str()), ApiDocV2::openapi()),
        )
}

pub fn init() -> Router {
    let routers_v1 = init_v1();
    routers_v1.merge(init_v2())
}
