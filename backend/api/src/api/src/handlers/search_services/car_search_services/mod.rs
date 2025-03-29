use super::search_requests::{
    SearchByFIORequest, SearchByGosNumRequest, SearchByPassportRequest,
};
use super::StatusResponse;
use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use models::Car;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod search_by_fio;
pub mod search_by_gos_num_mask;
pub mod search_by_passport;

pub use search_by_fio::{handle_search_car_by_fio, __path_handle_search_car_by_fio};
pub use search_by_passport::{handle_search_car_by_passport, __path_handle_search_car_by_passport};
pub use search_by_gos_num_mask::{handle_search_car_by_gos_num_mask, __path_handle_search_car_by_gos_num_mask};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct CarSearcherResponse {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
    #[schema(example = json!([]))]
    pub cars: Vec<Car>,
}
