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
    async fn get_car_route(
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
            .insert_track_info(gos_num, user_login, date)
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
                "Successfully generated route with {} points for vehicle {}",
                data.len(),
                gos_num
            );
            Ok(Some(data))
        }
    }
}
