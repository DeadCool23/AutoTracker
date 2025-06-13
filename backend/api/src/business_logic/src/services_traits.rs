use super::error::ServiceError;
use async_trait::async_trait;

use models::{Camera, Car, Document, Location, PointData, TrackInfo, User};

// # Сервис авторизации
// ===========================================

#[async_trait]
pub trait Authorizer: Send + Sync {
    async fn auth(&self, email: &String, pswd: &String) -> Result<User, ServiceError>;
    async fn register(
        &self,
        firstname: &String,
        surname: &String,
        lastname: Option<String>,
        email: &String,
        pswd: &String,
        rep_pswd: &String,
    ) -> Result<(), ServiceError>;
    async fn passport_confirm(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), ServiceError>;
}

// # Сервисы поиска
// ===========================================

#[async_trait]
pub trait CarSearcher: Send + Sync {
    async fn search_car(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
        passport: Option<Document>,
        gos_num_mask: Option<String>,
    ) -> Result<Vec<Car>, ServiceError>;
    async fn search_cars_by_owner_fio(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
    ) -> Result<Vec<Car>, ServiceError>;
    async fn search_cars_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, ServiceError>;
    async fn search_cars_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<Car>, ServiceError>;
}

#[async_trait]
pub trait TrackInfoSearcher: Send + Sync {
    async fn search_track_info(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
        passport: Option<Document>,
        gos_num_mask: Option<String>,
        date: Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    async fn search_track_info_by_owner_fio(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    async fn search_track_info_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    async fn search_track_info_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
    async fn search_track_info_by_date(
        &self,
        date: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError>;
}

pub trait Searcher: CarSearcher + TrackInfoSearcher {}

// # Сервис путей
// ===========================================

#[async_trait]
pub trait RouteGetter: Send + Sync {
    async fn get_car_route(
        &self,
        gos_num: &String,
        user_login: &String,
        date: &String,
    ) -> Result<Option<Vec<PointData>>, ServiceError>;
}

// # Сервис отправки изображений
// ===========================================

#[async_trait]
pub trait SnapSender: Send + Sync {
    async fn insert_snap(
        &self,
        camera: &Camera,
        speed: Option<u16>,
        time: &String,
        date: &String,
        gos_num: &String,
    ) -> Result<(), ServiceError>;
}

// # Сервис получения данных с камеры
// ===========================================

#[async_trait]
pub trait CameraDataGetter: Send + Sync {
    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, ServiceError>;
    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, ServiceError>;
    async fn get_avg_speed_of_car_on_camera_by_gos_num(
        &self,
        gos_num: &String,
        location: &Location,
    ) -> Result<f64, ServiceError>;
}
