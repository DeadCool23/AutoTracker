use super::validator::Validator;
use crate::error::ServiceError;
use data_access::repositories_traits::SnapRepository;
use models::{Camera, Snap};

use crate::services_traits;

pub struct SnapSendService {
    snap_repo: Box<dyn SnapRepository>,
}

impl SnapSendService {
    pub fn from(snap_repo: Box<dyn SnapRepository>) -> Self {
        SnapSendService { snap_repo }
    }
}

unsafe impl Send for SnapSendService {}
unsafe impl Sync for SnapSendService {}

impl services_traits::SnapSender for SnapSendService {
    fn insert_snap(
        &self,
        camera: &Camera,
        time: &String,
        date: &String,
        gos_num: &String,
    ) -> Result<(), ServiceError> {
        if !Validator::is_valid_gos_num(gos_num) {
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }
        if !Validator::is_valid_date(date) {
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }
        if !Validator::is_valid_time(time) {
            return Err(ServiceError::InvalidDataError("time".to_string()));
        }

        let _snap = Snap {
            camera: *camera,
            time: time.clone(),
            date: date.clone(),
            gos_num: gos_num.clone(),
        };
        self.snap_repo.insert_snap(&_snap)?;
        Ok(())
    }
}
