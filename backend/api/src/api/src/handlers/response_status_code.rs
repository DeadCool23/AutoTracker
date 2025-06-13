use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(non_camel_case_types)]
pub enum ResponseStatusCodeType {
    INVALID_DATA,
    EXIST_DATA,
    NOT_FOUNDED_DATA,
}

#[allow(non_camel_case_types)]
#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub enum ResponseStatusCode {
    OK = 0,
    INVALID_DATE = 1001,
    INVALID_GOS_NUM = 1002,
    INVALID_GOS_NUM_MASK = 1003,

    INVALID_AUTH_DATA = 2000,
    INVALID_EMAIL = 2001,
    INVALID_PSWD = 2002,
    INVALID_PSWDS = 2003,
    INVALID_PASSPORT = 2004,
    PASSPORT_EXIST = 2005,
    INVALID_TIME = 2006,

    EMAIL_EXIST = 3001,
    EMAIL_NOT_FOUNDED = 3002,

    CAMERA_NOT_FOUNDED = 4002,
    AVG_SPEED_NOT_FOUNDED = 4003,

    UNKNOWN_ERROR = 9999,
}

impl ResponseStatusCode {
    fn from_invalid_data(err: &str) -> Self {
        match err {
            "date" => ResponseStatusCode::INVALID_DATE,
            "gos number" => ResponseStatusCode::INVALID_GOS_NUM,
            "gos number mask" => ResponseStatusCode::INVALID_GOS_NUM_MASK,
            "email or password" => ResponseStatusCode::INVALID_AUTH_DATA,
            "email" => ResponseStatusCode::INVALID_EMAIL,
            "password" => ResponseStatusCode::INVALID_PSWD,
            "passwords unmatch" => ResponseStatusCode::INVALID_PSWDS,
            "time" => ResponseStatusCode::INVALID_TIME,
            "passport" => ResponseStatusCode::INVALID_PASSPORT,
            _ => ResponseStatusCode::UNKNOWN_ERROR,
        }
    }

    fn from_exist_data(err: &str) -> Self {
        match err {
            "email" => ResponseStatusCode::EMAIL_EXIST,
            "passport" => ResponseStatusCode::PASSPORT_EXIST,
            _ => ResponseStatusCode::UNKNOWN_ERROR,
        }
    }

    fn from_not_founded_data(err: &str) -> Self {
        match err {
            "email" => ResponseStatusCode::EMAIL_NOT_FOUNDED,
            "camera" => ResponseStatusCode::CAMERA_NOT_FOUNDED,
            "average speed" => ResponseStatusCode::AVG_SPEED_NOT_FOUNDED,
            _ => ResponseStatusCode::UNKNOWN_ERROR,
        }
    }

    pub fn from(err: &str, err_type: ResponseStatusCodeType) -> Self {
        match err_type {
            ResponseStatusCodeType::INVALID_DATA => ResponseStatusCode::from_invalid_data(err),
            ResponseStatusCodeType::EXIST_DATA => ResponseStatusCode::from_exist_data(err),
            ResponseStatusCodeType::NOT_FOUNDED_DATA => {
                ResponseStatusCode::from_not_founded_data(err)
            }
        }
    }
}
