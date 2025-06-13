use crate::error::DataAccessError;
use crate::repositories_traits::{SnapRepository, TandemRepoForTransfer, VolatileSnapRepo};
use async_trait::async_trait;
use models::Snap;

pub struct TandemSnapRepo {
    threshold: usize,
    main_storage: Box<dyn VolatileSnapRepo>,
    extra_storage: Box<dyn VolatileSnapRepo>,
}

impl TandemSnapRepo {
    pub fn from(
        main_storage: Box<dyn VolatileSnapRepo + Send + Sync>,
        extra_storage: Box<dyn VolatileSnapRepo + Send + Sync>,
    ) -> Self {
        log::info!("Creating TandemSnapRepo with default threshold (5)");
        TandemSnapRepo {
            main_storage,
            extra_storage,
            threshold: 5,
        }
    }

    pub fn with_threshold(mut self, threshold: usize) -> Self {
        log::info!("Setting new threshold: {}", threshold);
        self.threshold = threshold;
        self
    }

    async fn check_and_transfer(&self) -> Result<(), DataAccessError> {
        log::debug!(
            "Checking if transfer is needed (threshold: {})",
            self.threshold
        );
        let count = self.extra_storage.get_all_snaps().await?.len();
        log::debug!("Current extra_storage size: {}", count);

        if count >= self.threshold {
            log::info!(
                "Threshold reached ({} >= {}), initiating transfer",
                count,
                self.threshold
            );
            self.transfer().await?;
        }
        Ok(())
    }
}

#[async_trait]
impl TandemRepoForTransfer for TandemSnapRepo {
    async fn transfer(&self) -> Result<(), DataAccessError> {
        log::info!("Starting transfer from extra_storage to main_storage");

        log::debug!("Getting all snaps from extra_storage");
        let snaps = self.extra_storage.get_all_snaps().await?;
        log::debug!("Found {} snaps to transfer", snaps.len());

        log::info!("Inserting {} snaps into main_storage", snaps.len());
        let _ = self.main_storage.insert_snaps(&snaps).await?;

        log::info!("Clearing extra_storage");
        self.extra_storage.clear_snaps().await?;

        log::info!("Transfer completed successfully");
        Ok(())
    }
}

#[async_trait]
impl SnapRepository for TandemSnapRepo {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting snap into extra_storage (vehicle: {}, date: {}, time: {})",
            snap.gos_num,
            snap.date,
            snap.time
        );

        let result = self.extra_storage.insert_snap(snap).await;

        log::debug!("Checking transfer conditions after insert");
        if let Err(e) = self.check_and_transfer().await {
            log::error!("Failed to check/transfer after insert: {}", e);
            return Err(e);
        }

        result
    }

    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting snaps for vehicle {} on date {}", gos_number, date);

        log::debug!("Querying main_storage");
        let mut snaps = self
            .main_storage
            .get_car_snaps_by_date(gos_number, date)
            .await?;
        log::debug!("Found {} snaps in main_storage", snaps.len());

        log::debug!("Querying extra_storage");
        let mut extra_snaps = self
            .extra_storage
            .get_car_snaps_by_date(gos_number, date)
            .await?;
        log::debug!("Found {} snaps in extra_storage", extra_snaps.len());

        snaps.append(&mut extra_snaps);
        log::info!("Returning combined result of {} snaps", snaps.len());
        Ok(snaps)
    }
}
