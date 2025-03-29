use lazy_static::lazy_static;

lazy_static! {
    // Swagger
    pub static ref DOCS_PATH: String = "/docs".to_string();
    pub static ref OPENAPI_DOCS_PATH: String = "/api-docs/openapi.json".to_string();
    // Main path
    pub static ref CAR_PATH: String = "/car".to_string();
    pub static ref USER_PATH: String = "/user".to_string();
    pub static ref SNAP_PATH: String = "/snap".to_string();
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

    // Search path
    pub static ref CAR_SEARCH_SERVICE_PATH: String = format!("{}/search", CAR_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_SERVICE_PATH: String = format!("{}/search", TRACK_INFO_PATH.as_str());

    // Car search
    pub static ref CAR_SEARCH_BY_FIO_SERVICE_PATH: String = format!("{}/by-fio", CAR_SEARCH_SERVICE_PATH.as_str());
    pub static ref CAR_SEARCH_BY_PASSPORT_SERVICE_PATH: String = format!("{}/by-passport", CAR_SEARCH_SERVICE_PATH.as_str());
    pub static ref CAR_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH: String = format!("{}/by-gos-num-mask", CAR_SEARCH_SERVICE_PATH.as_str());
    
    // Track Info search
    pub static ref TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH: String = format!("{}/by-fio", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_DATE_SERVICE_PATH: String = format!("{}/by-date", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_PASSPORT_SERVICE_PATH: String = format!("{}/by-passport", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
    pub static ref TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH: String = format!("{}/by-gos-num-mask", TRACK_INFO_SEARCH_SERVICE_PATH.as_str());
}
