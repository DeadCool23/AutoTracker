use data_access::repositories_traits::{CarRepository, TrackInfoRepository};

use crate::error::ServiceError;
use crate::services_traits::{CarSearcher, Searcher, TrackInfoSearcher};
use async_trait::async_trait;
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

#[async_trait]
impl TrackInfoSearcher for SearchService {
    async fn search_track_info(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
        passport: Option<Document>,
        gos_num_mask: Option<String>,
        date: Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        log::info!(
            "Searching track info records by filters: {:?} {:?} {:?} {:?} {:?} {:?}",
            firstname.as_deref(),
            surname.as_deref(),
            lastname.as_deref(),
            passport,
            gos_num_mask.as_deref(),
            date.as_deref(),
        );

        if let Some(gsm) = &gos_num_mask {
            if !Validator::is_valid_gos_num_mask(&gsm) {
                log::warn!("Invalid gos number mask format: {}", &gsm);
                return Err(ServiceError::InvalidDataError(
                    "gos number mask".to_string(),
                ));
            }
        }

        if let Some(psprt) = &passport {
            if !Validator::is_valid_passport(&psprt) {
                log::warn!("Invalid passport format: {:#?}", &psprt);
                return Err(ServiceError::InvalidDataError("passport".to_string()));
            }
        }

        if let Some(dt) = &date {
            if !Validator::is_valid_date(dt) {
                log::warn!("Invalid date format: {}", dt);
                return Err(ServiceError::InvalidDataError("date".to_string()));
            }
        }

        let track_infos = self
            .track_info_repo
            .get_tracks_info_by_filters(
                firstname.as_deref(),
                surname.as_deref(),
                lastname.as_deref(),
                passport,
                gos_num_mask.as_deref(),
                date.as_deref(),
            )
            .await?;

        log::debug!("Found {} track info records by filters", track_infos.len());
        Ok(track_infos)
    }

    async fn search_track_info_by_owner_fio(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        log::info!(
            "Searching track info by owner FIO: {} {} {}",
            firstname.as_deref().unwrap_or(""),
            surname.as_deref().unwrap_or(""),
            lastname.as_deref().unwrap_or("")
        );

        let track_infos = self
            .track_info_repo
            .get_track_info_by_user_fio(
                firstname.as_deref(),
                surname.as_deref(),
                lastname.as_deref(),
            )
            .await?;

        log::debug!("Found {} track info records by FIO", track_infos.len());
        Ok(track_infos)
    }

    async fn search_track_info_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        log::info!("Searching track info by passport: {}", passport.number);

        if !Validator::is_valid_passport(passport) {
            log::warn!("Invalid passport format: {:#?}", passport);
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        let track_infos = self
            .track_info_repo
            .get_track_info_by_user_passport(passport)
            .await?;

        log::debug!("Found {} track info records by passport", track_infos.len());
        Ok(track_infos)
    }

    async fn search_track_info_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        log::info!("Searching track info by gos number mask: {}", gos_num_mask);

        if !Validator::is_valid_gos_num_mask(gos_num_mask) {
            log::warn!("Invalid gos number mask format: {}", gos_num_mask);
            return Err(ServiceError::InvalidDataError(
                "gos number mask".to_string(),
            ));
        }

        let track_infos = self
            .track_info_repo
            .get_track_info_by_car_gos_number_mask(gos_num_mask)
            .await?;

        log::debug!(
            "Found {} track info records by gos number mask",
            track_infos.len()
        );
        Ok(track_infos)
    }

    async fn search_track_info_by_date(
        &self,
        date: &String,
    ) -> Result<Vec<TrackInfo>, ServiceError> {
        log::info!("Searching track info by date: {}", date);

        if !Validator::is_valid_date(date) {
            log::warn!("Invalid date format: {}", date);
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }

        let track_infos = self.track_info_repo.get_track_info_by_date(date).await?;
        log::debug!(
            "Found {} track info records for date {}",
            track_infos.len(),
            date
        );
        Ok(track_infos)
    }
}

#[async_trait]
impl CarSearcher for SearchService {
    async fn search_car(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
        passport: Option<Document>,
        gos_num_mask: Option<String>,
    ) -> Result<Vec<Car>, ServiceError> {
        log::info!(
            "Searching cars by filters: {:?} {:?} {:?} {:?} {:?}",
            firstname.as_deref(),
            surname.as_deref(),
            lastname.as_deref(),
            passport,
            gos_num_mask.as_deref()
        );

        if let Some(gsm) = &gos_num_mask {
            if !Validator::is_valid_gos_num_mask(&gsm) {
                log::warn!("Invalid gos number mask format: {}", &gsm);
                return Err(ServiceError::InvalidDataError(
                    "gos number mask".to_string(),
                ));
            }
        }

        if let Some(psprt) = &passport {
            if !Validator::is_valid_passport(&psprt) {
                log::warn!("Invalid passport format: {:#?}", &psprt);
                return Err(ServiceError::InvalidDataError("passport".to_string()));
            }
        }

        let cars = self
            .car_repo
            .get_cars_by_filters(
                firstname.as_deref(),
                surname.as_deref(),
                lastname.as_deref(),
                passport,
                gos_num_mask.as_deref(),
            )
            .await?;

        log::debug!("Found {} cars by filters", cars.len());
        Ok(cars)
    }

    async fn search_cars_by_owner_fio(
        &self,
        firstname: Option<String>,
        surname: Option<String>,
        lastname: Option<String>,
    ) -> Result<Vec<Car>, ServiceError> {
        log::info!(
            "Searching cars by owner FIO: {} {} {}",
            firstname.as_deref().unwrap_or(""),
            surname.as_deref().unwrap_or(""),
            lastname.as_deref().unwrap_or("")
        );

        let cars = self
            .car_repo
            .get_car_by_owner_fio(
                firstname.as_deref(),
                surname.as_deref(),
                lastname.as_deref(),
            )
            .await?;

        log::debug!("Found {} cars by FIO", cars.len());
        Ok(cars)
    }

    async fn search_cars_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, ServiceError> {
        log::info!("Searching cars by passport: {}", passport.number);

        if !Validator::is_valid_passport(passport) {
            log::warn!("Invalid passport format: {:#?}", passport);
            return Err(ServiceError::InvalidDataError("passport".to_string()));
        }

        let cars = self.car_repo.get_car_by_owner_passport(passport).await?;
        log::debug!("Found {} cars by passport", cars.len());
        Ok(cars)
    }

    async fn search_cars_by_gos_num_mask(
        &self,
        gos_num_mask: &String,
    ) -> Result<Vec<Car>, ServiceError> {
        log::info!("Searching cars by gos number mask: {}", gos_num_mask);

        if !Validator::is_valid_gos_num_mask(gos_num_mask) {
            log::warn!("Invalid gos number mask format: {}", gos_num_mask);
            return Err(ServiceError::InvalidDataError(
                "gos number mask".to_string(),
            ));
        }

        let cars = self
            .car_repo
            .get_car_by_gos_number_mask(gos_num_mask)
            .await?;

        log::debug!("Found {} cars by gos number mask", cars.len());
        Ok(cars)
    }
}
