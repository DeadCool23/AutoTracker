use super::validator::Validator;
use crate::error::ServiceError;
use crate::services_traits::RouteGetter;
use async_trait::async_trait;
use models::PointData;

use data_access::repositories_traits::{SnapRepository, TrackInfoRepository, UserRepository};

pub struct RouteService {
    user_repo: Box<dyn UserRepository>,
    snap_repo: Box<dyn SnapRepository>,
    track_info_repo: Box<dyn TrackInfoRepository>,
}

impl RouteService {
    pub fn from(
        user_repo: Box<dyn UserRepository>,
        snap_repo: Box<dyn SnapRepository>,
        track_info_repo: Box<dyn TrackInfoRepository>,
    ) -> Self {
        RouteService {
            user_repo,
            snap_repo,
            track_info_repo,
        }
    }
}

unsafe impl Send for RouteService {}
unsafe impl Sync for RouteService {}

#[async_trait]
impl RouteGetter for RouteService {
    async fn is_car_owned_by_user(
        &self,
        user_id: usize,
        gos_num: &String,
    ) -> Result<bool, ServiceError> {
        log::info!(
            "Starting checking is user {} owned car {}",
            user_id,
            gos_num
        );

        if !Validator::is_valid_gos_num(gos_num) {
            log::warn!("Invalid vehicle number format: {}", gos_num);
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }

        let ugos_nums = self.user_repo.get_user_cars_gos_nums(user_id).await?;

        let mut is_owner = false;
        for ugos_num in ugos_nums.iter().as_ref() {
            if ugos_num == gos_num {
                is_owner = true;
                break;
            }
        }

        Ok(is_owner)
    }

    async fn get_car_route_with_user_email(
        &self,
        gos_num: &String,
        user_login: &String,
        date: &String,
    ) -> Result<Option<Vec<PointData>>, ServiceError> {
        log::info!(
            "Starting route request for vehicle {} by user {} on date {}",
            gos_num,
            user_login,
            date
        );

        if !Validator::is_valid_gos_num(gos_num) {
            log::warn!("Invalid vehicle number format: {}", gos_num);
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }
        if !Validator::is_valid_date(date) {
            log::warn!("Invalid date format: {}", date);
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }
        if !Validator::is_valid_email(user_login) {
            log::warn!("Invalid user login format: {}", user_login);
            return Err(ServiceError::InvalidDataError("email".to_string()));
        }

        log::debug!("Check login {}", user_login);
        let is_exist = self
            .user_repo
            .get_user_by_email(&user_login)
            .await?
            .is_none();

        if is_exist {
            log::warn!("User with login: {} not founded", user_login);
            return Err(ServiceError::NotFoundError("email".to_string()));
        }

        log::debug!(
            "Fetching snap data for vehicle {} on date {}",
            gos_num,
            date
        );
        let mut snaps = self.snap_repo.get_car_snaps_by_date(gos_num, date).await?;
        log::debug!("Found {} snap points", snaps.len());

        snaps.sort_by(|x, y| x.time.cmp(&y.time));
        log::debug!("Snap points sorted by time");

        log::info!("Recording track info request for user {}", user_login);
        self.track_info_repo
            .insert_track_info_by_user_email(gos_num, user_login, date)
            .await?;

        let data: Vec<_> = snaps
            .iter()
            .map(|x| PointData {
                speed: x.speed,
                cords: x.camera.location,
            })
            .collect();

        if data.is_empty() {
            log::warn!(
                "No location data found for vehicle {} on date {}",
                gos_num,
                date
            );
            Ok(None)
        } else {
            log::info!(
                "Successfully getted route with {} points for vehicle {}",
                data.len(),
                gos_num
            );
            Ok(Some(data))
        }
    }

    async fn get_car_route_with_user_id(
        &self,
        user_id: usize,
        gos_num: &String,
        date: &String,
    ) -> Result<Vec<PointData>, ServiceError> {
        log::info!(
            "Starting route request for vehicle {} by user {} on date {}",
            gos_num,
            user_id,
            date
        );

        if !Validator::is_valid_gos_num(gos_num) {
            log::warn!("Invalid vehicle number format: {}", gos_num);
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }
        if !Validator::is_valid_date(date) {
            log::warn!("Invalid date format: {}", date);
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }

        log::debug!("Check login {}", user_id);
        let is_exist = self.user_repo.get_user_by_id(user_id).await?.is_some();

        if !is_exist {
            log::warn!("User with login: {} not founded", user_id);
            return Err(ServiceError::NotFoundError("user_id".to_string()));
        }

        log::debug!(
            "Fetching snap data for vehicle {} on date {}",
            gos_num,
            date
        );
        let mut snaps = self.snap_repo.get_car_snaps_by_date(gos_num, date).await?;
        log::debug!("Found {} snap points", snaps.len());

        snaps.sort_by(|x, y| x.time.cmp(&y.time));
        log::debug!("Snap points sorted by time");

        log::info!("Recording track info request for user {}", user_id);
        self.track_info_repo
            .insert_track_info_by_user_id(gos_num, user_id, date)
            .await?;

        let data: Vec<_> = snaps
            .iter()
            .map(|x| PointData {
                speed: x.speed,
                cords: x.camera.location,
            })
            .collect();

        log::info!(
            "Successfully getted route with {} points for vehicle {}",
            data.len(),
            gos_num
        );
        Ok(data)
    }
}
