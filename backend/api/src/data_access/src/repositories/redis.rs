use crate::{
    error::DataAccessError,
    repositories_traits::{SnapRepoTransfer, SnapRepository},
};
use models::Snap;

pub struct RedisSnapRepoo {
    pub redis_client: redis::Client,
}

impl RedisSnapRepoo {
    pub fn new(redis_url: String) -> Result<Self, DataAccessError> {
        match redis::Client::open(redis_url) {
            Ok(client) => Ok(RedisSnapRepoo {
                redis_client: client,
            }),
            Err(e) => Err(DataAccessError::RedisDataBaseError(e)),
        }
    }
}

#[allow(unused_variables)]
impl SnapRepository for RedisSnapRepoo {
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

#[allow(unused_variables)]
impl SnapRepoTransfer for RedisSnapRepoo {
    fn get_all_snaps(&self) -> Result<Vec<Snap>, DataAccessError> {
        unimplemented!()
    }
    fn insert_snaps(&self, snaps: &Vec<Snap>) -> Result<(), DataAccessError> {
        unimplemented!()
    }
    fn clear_snaps(&self) -> Result<(), DataAccessError> {
        unimplemented!()
    }
}
