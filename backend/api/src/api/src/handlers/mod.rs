use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;

use models::Location;

pub use di_container::error::ServiceError;
pub use di_container::{BLServices, BUSINESS_SERVICES};

use auth_services::{
    auth_service::__path_handle_auth, passport_confirm_service::__path_handle_passport_conf,
    registration_service::__path_handle_reg,
};
use auth_services::{
    auth_service::{AuthRequest, AuthResponse},
    passport_confirm_service::PassportConfRequest,
    registration_service::RegRequest,
};
use camera_service::{
    camera_get_by_cords_service::__path_handle_get_camera_by_cords,
    camera_get_by_id_service::__path_handle_get_camera_by_id,
    camera_response::CameraResponse,
    get_avg_speed_service::{
        AvgSpeedRequest, AvgSpeedResponse, __path_handle_get_avg_speed_for_car_on_camera,
    },
};
use route_get_service::__path_handle_route;
use route_get_service::{RouteRequest, RouteResponse};
use snap_send_service::SnapSendRequest;
use snap_send_service::__path_handle_snap_send;

use search_services::{
    car_search_services::{
        CarSearcherResponse, SearchCarByFilterRequest, __path_handle_search_car_by_fio,
        __path_handle_search_car_by_gos_num_mask, __path_handle_search_car_by_passport,
        __path_handle_search_cars_by_filters,
    },
    search_requests::*,
    track_info_search_services::{
        SearchTrackInfoByFilterRequest, TrackInfoSearcherResponse,
        __path_handle_search_track_info_by_date, __path_handle_search_track_info_by_filters,
        __path_handle_search_track_info_by_fio, __path_handle_search_track_info_by_gos_num_mask,
        __path_handle_search_track_info_by_passport,
    },
};

pub mod response_status_code;
mod status_response;

pub use response_status_code::{ResponseStatusCode, ResponseStatusCodeType};
pub use status_response::StatusResponse;

pub mod auth_services;
pub mod camera_service;
pub mod route_get_service;
pub mod search_services;
pub mod snap_send_service;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AUTOTRACKER API",
        version = "0.1.0",
        description="API сервиса отслеживания марщрутов AutoTracker",
    ),
    servers(
        (url = "http://127.0.0.1:9887", description = "Local server"),
        (url = "http://{url}:9887", description = "Local network API",
            variables(
                ("url" = (default = "0.0.0.0", description = "Default local network")),
            )
        ),
    ),
    paths(
        handle_route, 

        handle_auth,
        handle_reg,
        handle_passport_conf, 
        
        handle_snap_send,

        handle_search_cars_by_filters,
        handle_search_car_by_fio,
        handle_search_car_by_passport,
        handle_search_car_by_gos_num_mask,

        handle_search_track_info_by_filters,
        handle_search_track_info_by_fio,
        handle_search_track_info_by_passport,
        handle_search_track_info_by_gos_num_mask,
        handle_search_track_info_by_date,

        handle_get_camera_by_id,
        handle_get_camera_by_cords,
        handle_get_avg_speed_for_car_on_camera,
    ),
    components(schemas(
        RouteRequest, RouteResponse, AuthRequest, AuthResponse, Location,
        RegRequest, PassportConfRequest, SnapSendRequest, ResponseWithoutData,
        CarSearcherResponse, TrackInfoSearcherResponse, SearchByFIORequest,
        SearchByDateRequest, SearchByGosNumRequest, SearchByPassportRequest, CameraResponse,
        AvgSpeedRequest, AvgSpeedResponse, SearchTrackInfoByFilterRequest, SearchCarByFilterRequest
    )),
    tags(
        (name = "route", description = "Получение маршрута"),
        (name = "auth", description = "Авторизация"),
        (name = "snap", description = "Снимки"),
        (name = "search", description = "Поисковик"),
        (name = "car", description = "Автомобили"),
        (name = "camera", description = "Камера"),
        (name = "track-info", description = "Информация об отслеживании"),
    )
)]
pub struct ApiDoc;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct ResponseWithoutData {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
}
