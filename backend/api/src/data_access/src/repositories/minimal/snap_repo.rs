use crate::error::DataAccessError;
use crate::repositories_traits::SnapRepository;
use async_trait::async_trait;
use models::Snap;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct MapSnapRepository {
    snaps: Arc<RwLock<HashMap<String, Vec<Snap>>>>,
}

impl MapSnapRepository {
    pub fn new() -> Self {
        Self {
            snaps: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SnapRepository for MapSnapRepository {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        let mut snaps_map = self.snaps.write().map_err(|e| {
            DataAccessError::ConnectionError(
                format!("Failed to acquire write lock: {}", e).to_string(),
            )
        })?;

        let entry = snaps_map
            .entry(snap.gos_num.clone())
            .or_insert_with(Vec::new);
        entry.push(snap.clone());

        entry.sort_by(|a, b| {
            let date_time_a = format!("{} {}", a.date, a.time);
            let date_time_b = format!("{} {}", b.date, b.time);
            date_time_a.cmp(&date_time_b)
        });

        Ok(())
    }

    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        let snaps_map = self.snaps.read().map_err(|e| {
            DataAccessError::ConnectionError(
                format!("Failed to acquire read lock: {}", e).to_string(),
            )
        })?;

        if let Some(snaps) = snaps_map.get(gos_number) {
            let filtered_snaps: Vec<Snap> = snaps
                .iter()
                .filter(|snap| snap.date == date)
                .cloned()
                .collect();

            Ok(filtered_snaps)
        } else {
            Ok(Vec::new())
        }
    }
}
