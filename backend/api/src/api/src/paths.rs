use lazy_static::lazy_static;

fn api_vpath(version: u8) -> String {
    format!("/api/v{version}")
}

pub fn vpath(version: u8, path: &str) -> String {
    format!("{}{}", api_vpath(version), path)
}

lazy_static! {
    // Swagger
    pub static ref DOCS_PATH: String = "/docs".to_string();
    pub static ref OPENAPI_DOCS_PATH: String = "/api-docs/openapi.json".to_string();
    // Main path
    pub static ref CAR_PATH: String = "/car".to_string();
    pub static ref USER_PATH: String = "/user".to_string();
    pub static ref SNAP_PATH: String = "/snap".to_string();
    pub static ref CAMERA_PATH: String = "/camera".to_string();
    pub static ref SEARCH_PATH: String = "/search".to_string();
    pub static ref TRACK_INFO_PATH: String = "/track-info".to_string();

    // Route path
    pub static ref ROUTE_GET_SERVICE_PATH: String = format!("{}/route", CAR_PATH.as_str());

    // Auth path
    pub static ref AUTH_SERVICE_PATH: String = format!("{}/auth", USER_PATH.as_str());
    pub static ref REG_SERVICE_PATH: String = format!("{}/registr", USER_PATH.as_str());
    pub static ref PASSPORT_CONF_SERVICE_PATH: String =
        format!("{}/passport-confirm", USER_PATH.as_str());

    // Snap path
    pub static ref SNAP_SEND_SERVICE_PATH: String =
        format!("{}/send", SNAP_PATH.as_str());

    // Camera path
    pub static ref CAMERA_GET_BY_ID_SERVICE_PATH: String =
        format!("{}/{{id}}", CAMERA_PATH.as_str());
    pub static ref CAMERA_GET_BY_CORDS_SERVICE_PATH: String =
        format!("{}/by-location", CAMERA_PATH.as_str());
    pub static ref GET_AVG_SPEED_ON_CAMERA_PATH: String =
        format!("{}/avg-speed", CAMERA_PATH.as_str());

    // Search path
    pub static ref CAR_SEARCH_SERVICE_PATH: String = format!("{}{}", CAR_PATH.as_str(), SEARCH_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_SERVICE_PATH: String = format!("{}{}", TRACK_INFO_PATH.as_str(), SEARCH_PATH.as_str());

    // Car search
    pub static ref CAR_SEARCH_BY_FIO_SERVICE_PATH: String = format!("{}/by-fio", CAR_SEARCH_SERVICE_PATH.as_str());
    pub static ref CAR_SEARCH_BY_PASSPORT_SERVICE_PATH: String = format!("{}/by-passport", CAR_SEARCH_SERVICE_PATH.as_str());
    pub static ref CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH: String = format!("{}/by-gos-num-mask", CAR_SEARCH_SERVICE_PATH.as_str());

    // Track Info search
    pub static ref TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH: String = format!("{}/by-fio", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH: String = format!("{}/by-date", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH: String = format!("{}/by-passport", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH: String = format!("{}/by-gos-num-mask", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());

    // Main paths v2
    pub static ref CARS_PATH: String = "/cars".to_string();
    pub static ref USERS_PATH: String = "/users".to_string();
    pub static ref SNAPS_PATH: String = "/snaps".to_string();
    pub static ref CAMERAS_PATH: String = "/cameras".to_string();
    pub static ref TRACK_INFOS_PATH: String = "/track-infos".to_string();

    // Auth path v2
    pub static ref AUTH_SERVICE_V2_PATH: String = format!("{}/login", USERS_PATH.as_str());
    pub static ref USER_ID_SERVICES_V2_PATH: String = format!("{}/{{id}}", USERS_PATH.as_str());
    pub static ref REG_SERVICE_V2_PATH: String = format!("{}/registr", USERS_PATH.as_str());
    pub static ref PASSPORT_CONF_SERVICE_V2_PATH: String =
        format!("{}/{{id}}/passport", USERS_PATH.as_str());

    // Search path v2
    pub static ref CARS_SEARCH_SERVICE_PATH_V2: String = format!("{}{}", CARS_PATH.as_str(), SEARCH_PATH.as_str());
    pub static ref TRACK_INFOS_SEARCH_SERVICE_PATH_V2: String = format!("{}{}", TRACK_INFOS_PATH.as_str(), SEARCH_PATH.as_str());

    // Route get path v2
    pub static ref ROUTE_GET_SERVICE_PATH_V2: String = format!("{}/route", CARS_PATH.as_str());
}
