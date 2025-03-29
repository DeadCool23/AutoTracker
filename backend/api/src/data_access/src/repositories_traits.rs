use models::{Car, Camera, Document, Snap, TrackInfo, User};

use super::error::DataAccessError;

pub trait UserRepository {
    fn get_user_by_auth_info(
        &self,
        email: &str,
        pswd: &str,
    ) -> Result<Option<User>, DataAccessError>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError>;
    fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<Vec<User>>, DataAccessError>;
    fn get_user_by_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Option<Vec<User>>, DataAccessError>;
    fn add_user(&self, user: &User) -> Result<(), DataAccessError>;
    fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError>;
}

pub trait CarRepository {
    fn get_car_by_gos_number_mask(&self, gos_number: &str) -> Result<Vec<Car>, DataAccessError>;
    fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError>;
    fn get_car_by_owner_passport(&self, passport: &Document) -> Result<Vec<Car>, DataAccessError>;
}

pub trait TrackInfoRepository {
    fn insert_track_info(
        &self,
        gos_num: &str,
        user_id: usize,
        route_date: &str,
    ) -> Result<(), DataAccessError>;
    fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError>;
    fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
    fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
    fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError>;
}

pub trait SnapRepository {
    fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError>;
    fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError>;
}

pub trait SnapRepoTransfer {
    fn get_all_snaps(&self) -> Result<Vec<Snap>, DataAccessError>;
    fn insert_snaps(&self, snaps: &Vec<Snap>) -> Result<(), DataAccessError>;
    fn clear_snaps(&self) -> Result<(), DataAccessError>;
}

pub trait VolatileSnapRepo: SnapRepository + SnapRepoTransfer {}

pub trait CameraRepository {
    fn get_camera_count(&self) -> Result<usize, DataAccessError>;
    fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError>;
}