use super::error::DataAccessError;
use async_trait::async_trait;
use models::{Camera, Car, Document, Location, Snap, TrackInfo, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_auth_info(
        &self,
        email: &str,
        pswd: &str,
    ) -> Result<Option<User>, DataAccessError>;
    async fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<User>, DataAccessError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError>;
    async fn insert_user(&self, user: &User, pswd: &str) -> Result<(), DataAccessError>;
    async fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError>;
}

#[async_trait]
pub trait CarRepository: Send + Sync {
    async fn get_cars_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError>;
    async fn get_car_by_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<Car>, DataAccessError>;
    async fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError>;
    async fn get_car_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, DataAccessError>;
}

#[async_trait]
pub trait TrackInfoRepository: Send + Sync {
    async fn insert_track_info(
        &self,
        gos_num: &str,
        user_login: &str,
        route_date: &str,
    ) -> Result<(), DataAccessError>;
    async fn get_tracks_info_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
    async fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError>;
    async fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
    async fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
    async fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
}

#[async_trait]
pub trait SnapRepository: Send + Sync {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError>;
    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError>;
}

#[async_trait]
pub trait SnapRepoTransfer {
    async fn get_all_snaps(&self) -> Result<Vec<Snap>, DataAccessError>;
    async fn insert_snaps(&self, snaps: &[Snap]) -> Result<(), DataAccessError>;
    async fn clear_snaps(&self) -> Result<(), DataAccessError>;
}

#[async_trait]
pub trait VolatileSnapRepo: SnapRepository + SnapRepoTransfer + Send + Sync {}

#[async_trait]
pub trait TandemRepoForTransfer {
    async fn transfer(&self) -> Result<(), DataAccessError>;
}

#[async_trait]
pub trait CameraRepository: Send + Sync {
    async fn get_camera_count(&self) -> Result<usize, DataAccessError>;
    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError>;
    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, DataAccessError>;
    async fn get_avg_speed_for_car_at_camera(
        &self,
        gos_num: &str,
        cam_id: usize,
    ) -> Result<f64, DataAccessError>;
}
