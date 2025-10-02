use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

pub use di_container::error::ServiceError;
pub use di_container::{CoreServices, ServicesContainer};

pub use super::response_status_code::{ResponseStatusCode, ResponseStatusCodeType};
pub use super::response_without_data::ResponseWithoutData;
pub use super::status_response::StatusResponse;

mod token_auth;

pub mod auth_services;
use auth_services::{
    auth_service::{AuthRequest, AuthResponse, __path_handle_auth_v2},
    passport_confirm_service::__path_handle_passport_conf_v2,
};

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
        handle_passport_conf_v2
    ),
    components(schemas(
        AuthRequest, AuthResponse, ResponseWithoutData

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
