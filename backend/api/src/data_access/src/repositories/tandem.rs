use crate::error::DataAccessError;
use crate::repositories_traits::{SnapRepository, VolatileSnapRepo};
use models::Snap;

pub struct TandemSnapRepo {
    pub main_storage: Box<dyn VolatileSnapRepo>,
    pub extra_storage: Box<dyn VolatileSnapRepo>,
}

impl TandemSnapRepo {
    pub fn new(
        main_storage: Box<dyn VolatileSnapRepo>,
        extra_storage: Box<dyn VolatileSnapRepo>,
    ) -> Self {
        TandemSnapRepo {
            main_storage,
            extra_storage,
        }
    }
}

#[allow(unused_variables)]
impl SnapRepository for TandemSnapRepo {
    fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        unimplemented!()
    }
    fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        unimplemented!()
    }
}
