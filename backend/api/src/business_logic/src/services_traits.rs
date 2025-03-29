use super::error::ServiceError;

use models::{Camera, Car, Document, Location, TrackInfo, User};

// # Сервис авторизации
// ===========================================

pub trait Authorizer: Send + Sync {
    fn auth(&self, email: &String, pswd: &String) -> Result<User, ServiceError>;
    fn register(
        &self,
        firstname: &String,
        surname: &String,
        lastname: &Option<String>,
        email: &String,
        pswd: &String,
        rep_pswd: &String,
    ) -> Result<(), ServiceError>;
    fn passport_confirm(&self, email: &String, passport: &Document) -> Result<(), ServiceError>;
}

// # Сервисы поиска
// ===========================================

pub trait CarSearcher: Send + Sync {
    fn search_cars_by_owner_fio(
        &self,
        firstname: &Option<String>,
        surname: &Option<String>,
        lastname: &Option<String>,
    ) -> Result<Vec<Car>, ServiceError>;
    fn search_cars_by_owner_passport(&self, passport: &Document) -> Result<Vec<Car>, ServiceError>;
    fn search_cars_by_gos_num_mask(&self, gos_num_mask: &String) -> Result<Vec<Car>, ServiceError>;
}

pub trait TrackInfoSearcher: Send + Sync {
    fn search_track_info_by_owner_fio(
        &self,
        firstname: &Option<String>,
        surname: &Option<String>,
        lastname: &Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    fn search_track_info_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    fn search_track_info_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    fn search_track_info_by_date(&self, date: &String) -> Result<Vec<TrackInfo>, ServiceError>;
}

pub trait Searcher: CarSearcher + TrackInfoSearcher {}

pub trait RouteGetter: Send + Sync {
    fn get_car_route(
        &self,
        gos_num: &String,
        user_id: usize,
        date: &String,
    ) -> Result<Option<Vec<Location>>, ServiceError>;
}

pub trait SnapSender: Send + Sync {
    fn insert_snap(
        &self,
        camera: &Camera,
        time: &String,
        date: &String,
        gos_num: &String,
    ) -> Result<(), ServiceError>;
}
