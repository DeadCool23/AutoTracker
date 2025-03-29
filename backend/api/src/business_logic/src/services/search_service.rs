use data_access::repositories_traits::{CarRepository, TrackInfoRepository};

use crate::error::ServiceError;
use crate::services_traits::{CarSearcher, Searcher, TrackInfoSearcher};
use models::{Car, Document, TrackInfo};

use super::validator::Validator;

pub struct SearchService {
    car_repo: Box<dyn CarRepository>,
    track_info_repo: Box<dyn TrackInfoRepository>,
}

impl SearchService {
    pub fn from(
        car_repo: Box<dyn CarRepository>,
        track_info_repo: Box<dyn TrackInfoRepository>,
    ) -> Self {
        SearchService {
            car_repo,
            track_info_repo,
        }
    }
}

impl Searcher for SearchService {}

unsafe impl Send for SearchService {}
unsafe impl Sync for SearchService {}

impl TrackInfoSearcher for SearchService {
    fn search_track_info_by_owner_fio(
        &self,
        firstname: &Option<String>,
        surname: &Option<String>,
        lastname: &Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        let track_infos = self.track_info_repo.get_track_info_by_user_fio(
            firstname.as_deref(),
            surname.as_deref(),
            lastname.as_deref(),
        )?;
        Ok(track_infos)
    }
    fn search_track_info_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        if !Validator::is_valid_passport(passport) {
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        let track_infos = self
            .track_info_repo
            .get_track_info_by_user_passport(passport)?;
        Ok(track_infos)
    }
    fn search_track_info_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        if !Validator::is_valid_gos_num_mask(gos_num_mask) {
            return Err(ServiceError::InvalidDataError("gos number mask".to_string()));
        }

        let track_infos = self
            .track_info_repo
            .get_track_info_by_car_gos_number_mask(&gos_num_mask)?;
        Ok(track_infos)
    }
    fn search_track_info_by_date(&self, date: &String) -> Result<Vec<TrackInfo>, ServiceError> {
        if !Validator::is_valid_date(&date) {
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }

        let track_infos = self.track_info_repo.get_track_info_by_date(&date)?;
        Ok(track_infos)
    }
}

impl CarSearcher for SearchService {
    fn search_cars_by_owner_fio(
        &self,
        firstname: &Option<String>,
        surname: &Option<String>,
        lastname: &Option<String>,
    ) -> Result<Vec<Car>, ServiceError> {
        let cars = self.car_repo.get_car_by_owner_fio(
            firstname.as_deref(),
            surname.as_deref(),
            lastname.as_deref(),
        )?;
        Ok(cars)
    }
    fn search_cars_by_owner_passport(&self, passport: &Document) -> Result<Vec<Car>, ServiceError> {
        if !Validator::is_valid_passport(passport) {
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        let cars = self.car_repo.get_car_by_owner_passport(passport)?;
        Ok(cars)
    }
    fn search_cars_by_gos_num_mask(&self, gos_num_mask: &String) -> Result<Vec<Car>, ServiceError> {
        if !Validator::is_valid_gos_num_mask(gos_num_mask) {
            return Err(ServiceError::InvalidDataError(
                "gos number mask".to_string(),
            ));
        }

        let cars = self.car_repo.get_car_by_gos_number_mask(&gos_num_mask)?;
        Ok(cars)
    }
}
