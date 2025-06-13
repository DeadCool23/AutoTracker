use crate::error::DataAccessError;
use crate::repositories_traits::{SnapRepoTransfer, SnapRepository, VolatileSnapRepo};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use models::{Camera, Location, Snap};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, QueryBuilder, Row,
};
use std::time::{Duration, Instant};

pub struct PgSnapRepo {
    pool: PgPool,
}

impl PgSnapRepo {
    pub async fn from(pg_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to PostgreSQL database");
        let pool = PgPoolOptions::new()
            .max_connections(super::MAX_CONNECTIONS)
            .acquire_timeout(super::CONNECTION_WATING_TIME)
            .connect(pg_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to PostgreSQL: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;
        log::info!("Successfully connected to PostgreSQL");
        Ok(PgSnapRepo { pool })
    }
}

impl PgSnapRepo {
    fn joined_tables_query() -> String {
        log::debug!("Generating joined tables query");
        "SELECT
            s.id,
            s.speed,
            s.gos_num,
            s.snap_datetime,
            c.id as camera_id,
            c.is_radar,
            c.longitude,
            c.latitude
        FROM CarSnapshot s
        JOIN Camera c ON s.camera_id = c.id"
            .to_string()
    }

    fn from_rows_to_snaps(rows: &[PgRow]) -> Result<Vec<Snap>, DataAccessError> {
        log::debug!("Converting {} database rows to Snap objects", rows.len());
        let mut snaps = Vec::new();
        for row in rows {
            let datetime: NaiveDateTime = row.get("snap_datetime");

            snaps.push(Snap {
                camera: Camera {
                    id: row.get::<i32, _>("camera_id") as usize,
                    is_radar: row.get("is_radar"),
                    location: Location {
                        longitude: row.get("longitude"),
                        latitude: row.get("latitude"),
                    },
                },
                speed: row
                    .get::<Option<i32>, _>("speed")
                    .and_then(|v| u16::try_from(v).ok()),
                gos_num: row.get("gos_num"),
                date: datetime.date().format("%d.%m.%Y").to_string(),
                time: datetime.time().format("%H:%M").to_string(),
            });
        }
        Ok(snaps)
    }
}

#[async_trait]
impl SnapRepository for PgSnapRepo {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );

        let datetime = NaiveDateTime::parse_from_str(
            &format!("{} {}", snap.date, snap.time),
            "%d.%m.%Y %H:%M",
        )
        .map_err(|e| {
            log::error!("Failed to parse datetime: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let query = "INSERT INTO CarSnapshot 
                    (camera_id, speed, snap_datetime, gos_num) 
                    VALUES ($1, $2, $3, $4)";

        sqlx::query(query)
            .bind(snap.camera.id as i32)
            .bind(snap.speed.map(|s| s as i32))
            .bind(datetime)
            .bind(&snap.gos_num)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to insert snap: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Successfully inserted snap");
        Ok(())
    }

    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting snaps for vehicle {} on date {}", gos_number, date);

        let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").map_err(|e| {
            log::error!("Invalid date format: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let start_datetime = date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let end_datetime = date.and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        let where_query = "
            WHERE s.gos_num = $1 
            AND s.snap_datetime BETWEEN $2::timestamp AND $3::timestamp
            ORDER BY s.snap_datetime
        ";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!("Executing query: {}", query);

        let rows = sqlx::query(query)
            .bind(gos_number)
            .bind(start_datetime)
            .bind(end_datetime)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::debug!("Found {} matching snaps", rows.len());
        Self::from_rows_to_snaps(&rows)
    }
}

impl PgSnapRepo {
    pub async fn delete_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Deleting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );

        let datetime = NaiveDateTime::parse_from_str(
            &format!("{} {}", snap.date, snap.time),
            "%d.%m.%Y %H:%M",
        )
        .map_err(|e| {
            log::error!("Failed to parse datetime: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let query = "DELETE FROM CarSnapshot 
                    WHERE camera_id = $1 
                    AND snap_datetime = $2::timestamp 
                    AND gos_num = $3";

        let affected = sqlx::query(query)
            .bind(snap.camera.id as i32)
            .bind(datetime)
            .bind(&snap.gos_num)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Delete failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?
            .rows_affected();

        if affected == 0 {
            log::warn!("No snap found to delete");
        } else {
            log::info!("Successfully deleted snap");
        }
        Ok(())
    }

    pub async fn delete_snaps(&self, snaps: &[Snap]) -> Result<(), DataAccessError> {
        for snap in snaps {
            self.delete_snap(&snap).await?
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl PgSnapRepo {
    pub async fn insert_snaps_by_one(&self, snaps: &[Snap]) -> Result<Duration, DataAccessError> {
        log::info!("Inserting batch of {} snaps one by one", snaps.len());
        let start_time = Instant::now();

        for (i, snap) in snaps.iter().enumerate() {
            let snap_start = Instant::now();
            self.insert_snap(snap).await?;
            log::debug!(
                "Inserted snap {}/{} in {}ms",
                i + 1,
                snaps.len(),
                snap_start.elapsed().as_millis()
            );
        }

        let total_time = start_time.elapsed();
        log::info!(
            "Successfully inserted {} snaps via single INSERTs. Total time: {:.2}s ({:.2}ms per snap)",
            snaps.len(),
            total_time.as_secs_f32(),
            total_time.as_millis() as f32 / snaps.len() as f32
        );
        Ok(total_time)
    }

    pub async fn insert_snaps_by_values(
        &self,
        snaps: &[Snap],
    ) -> Result<Duration, DataAccessError> {
        log::info!("Inserting batch of {} snaps using by VALUES", snaps.len());
        let start_time = std::time::Instant::now();

        for chunk in snaps.chunks(100) {
            let mut query_builder = QueryBuilder::new(
                "INSERT INTO CarSnapshot (camera_id, speed, snap_datetime, gos_num) ",
            );

            query_builder.push_values(chunk, |mut b, snap| {
                let datetime = NaiveDateTime::parse_from_str(
                    &format!("{} {}", snap.date, snap.time),
                    "%d.%m.%Y %H:%M",
                )
                .expect("Failed to parse datetime");

                b.push_bind(snap.camera.id as i32)
                    .push_bind(snap.speed.map(|s| s as i32))
                    .push_bind(datetime)
                    .push_bind(&snap.gos_num);
            });

            let query = query_builder.build();

            query.execute(&self.pool).await.map_err(|e| {
                log::error!("Failed to execute bulk insert: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;
        }

        let total_time = start_time.elapsed();
        log::info!(
            "Successfully inserted {} snaps via bulk VALUES. Total time: {:.2}s ({:.2}ms per snap)",
            snaps.len(),
            total_time.as_secs_f32(),
            total_time.as_millis() as f32 / snaps.len() as f32
        );
        Ok(total_time)
    }

    pub async fn insert_snaps_by_copy(&self, snaps: &[Snap]) -> Result<Duration, DataAccessError> {
        log::info!("Inserting batch of {} snaps using COPY", snaps.len());
        let start_time = std::time::Instant::now();

        let mut connection = self.pool.acquire().await.map_err(|e| {
            log::error!("Failed to acquire connection: {}", e);
            DataAccessError::PsqlDataBaseError(e)
        })?;

        let pg_conn: &mut sqlx::PgConnection = &mut *connection;

        let mut bytes = Vec::new();
        for snap in snaps {
            let datetime = NaiveDateTime::parse_from_str(
                &format!("{} {}", snap.date, snap.time),
                "%d.%m.%Y %H:%M",
            )
            .map_err(|e| {
                log::error!("Failed to parse datetime: {}", e);
                DataAccessError::InvalidInput(e.to_string())
            })?;

            let line = format!(
                "{}\t{}\t{}\t{}\n",
                snap.camera.id,
                snap.speed
                    .map(|s| s.to_string())
                    .unwrap_or("\\N".to_string()),
                datetime.format("%Y-%m-%d %H:%M:%S"),
                snap.gos_num
            );
            bytes.extend_from_slice(line.as_bytes());
        }

        {
            let mut copy = pg_conn
                .copy_in_raw(
                    "COPY CarSnapshot (camera_id, speed, snap_datetime, gos_num) FROM STDIN",
                )
                .await
                .map_err(|e| {
                    log::error!("Failed to start COPY: {}", e);
                    DataAccessError::PsqlDataBaseError(e)
                })?;

            copy.send(bytes).await.map_err(|e| {
                log::error!("Failed to send COPY data: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

            copy.finish().await.map_err(|e| {
                log::error!("Failed to finish COPY: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;
        }

        let total_time = start_time.elapsed();
        log::info!(
            "Successfully inserted {} snaps via COPY. Total time: {:.2}s ({:.2}ms per snap)",
            snaps.len(),
            total_time.as_secs_f32(),
            total_time.as_millis() as f32 / snaps.len() as f32
        );
        Ok(total_time)
    }
}

#[async_trait]
impl SnapRepoTransfer for PgSnapRepo {
    async fn get_all_snaps(&self) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting all snaps from database");
        let query = &Self::joined_tables_query();
        log::debug!("Executing query: {}", query);

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Retrieved {} snaps total", rows.len());
        Self::from_rows_to_snaps(&rows)
    }

    async fn insert_snaps(&self, snaps: &[Snap]) -> Result<(), DataAccessError> {
        log::info!("Starting batch insert of {} snaps", snaps.len());
        let _ = self.insert_snaps_by_one(snaps).await?;
        Ok(())
    }

    async fn clear_snaps(&self) -> Result<(), DataAccessError> {
        log::info!("Clearing all snaps from database");
        let query = "DELETE FROM CarSnapshot";

        let affected = sqlx::query(query)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Clear failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?
            .rows_affected();

        log::info!("Deleted {} snaps", affected);
        Ok(())
    }
}

impl VolatileSnapRepo for PgSnapRepo {}
