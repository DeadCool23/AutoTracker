use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

use di_container::error::ServiceError;
use di_container::{CoreServices, ServicesContainer};
use models::{Document, UserWithId, PointData};

pub use super::response_status_code::{ResponseStatusCode, ResponseStatusCodeType};
pub use super::status_response::StatusResponse;

mod token_auth;

pub mod auth_services;
use auth_services::{
    auth_service::{AuthRequest, AuthResponse, __path_handle_auth_v2},
    passport_confirm_service::__path_handle_passport_conf_v2,
    registration_service::{RegRequest, __path_handle_reg_v2},
    user_delete_service::__path_handle_delete_user_by_id_v2,
    user_get_service::__path_handle_get_user_by_id_v2,
};

pub mod search_services;
use search_services::{
    car_search_service::{
        CarSearcherResponse, SearchCarsRequest, __path_handle_search_cars_by_filters_with_offset_v2,
    },
    track_info_search_service::{
        SearchTrackInfoByFilterRequest, TrackInfoSearcherResponse,
        __path_handle_search_track_info_by_filters_with_offset_v2,
    },
};

pub mod route_get_service;
use route_get_service::{RouteRequest, __path_handle_route_v2};

const VERSION: u8 = 2;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "AUTOTRACKER API",
        version = "0.2.0",
        description="API сервиса отслеживания маршрутов AutoTracker",
    ),
    servers(
        (url = "http://127.0.0.1:9887", description = "Local server"),
        (url = "http://{url}:9887", description = "Local network API",
            variables(
                ("url" = (default = "0.0.0.0", description = "Default local network")),
            )
        ),
    ),
    modifiers(&SecurityAddon),
    paths(
        handle_auth_v2,
        handle_passport_conf_v2,
        handle_reg_v2,
        handle_get_user_by_id_v2,
        handle_delete_user_by_id_v2,

        handle_search_cars_by_filters_with_offset_v2,
        handle_search_track_info_by_filters_with_offset_v2,

        handle_route_v2,
    ),
    components(schemas(
        AuthRequest, AuthResponse,
        Document, RegRequest, UserWithId,
        SearchCarsRequest, CarSearcherResponse,
        TrackInfoSearcherResponse, SearchTrackInfoByFilterRequest,
        RouteRequest, PointData,
        StatusResponse,
    )),
    tags(
        (name = "route", description = "Получение маршрута"),
        (name = "auth", description = "Авторизация"),
        (name = "user", description = "Пользователь"),
        (name = "search", description = "Поисковик"),
        (name = "car", description = "Автомобили"),
        (name = "track-info", description = "Информация об отслеживании"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
