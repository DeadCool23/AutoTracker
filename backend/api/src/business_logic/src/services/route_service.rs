    use super::validator::Validator;
use crate::error::ServiceError;
use crate::services_traits::RouteGetter;

use data_access::repositories_traits::{SnapRepository, TrackInfoRepository};
use models::Location;

pub struct RouteService {
    snap_repo: Box<dyn SnapRepository>,
    track_info_repo: Box<dyn TrackInfoRepository>,
}

impl RouteService {
    pub fn from(
        snap_repo: Box<dyn SnapRepository>,
        track_info_repo: Box<dyn TrackInfoRepository>,
    ) -> Self {
        RouteService {
            snap_repo,
            track_info_repo,
        }
    }
}

unsafe impl Send for RouteService {}
unsafe impl Sync for RouteService {}

impl RouteGetter for RouteService {
    fn get_car_route(
        &self,
        gos_num: &String,
        user_id: usize,
        date: &String,
    ) -> Result<Option<Vec<Location>>, ServiceError> {
        if !Validator::is_valid_gos_num(gos_num) {
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }
        if !Validator::is_valid_date(date) {
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }
        let mut _snaps = self.snap_repo.get_car_snaps_by_date(gos_num, date)?;

        _snaps.sort_by(|x, y| x.time.cmp(&y.time));

        self.track_info_repo
            .insert_track_info(gos_num, user_id, date)?;

        let _locations: Vec<_> = _snaps.iter().map(|x| x.camera.location).collect();
        if _locations.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(_locations))
        }
    }
}
