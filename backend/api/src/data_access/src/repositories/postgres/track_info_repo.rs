use crate::error::DataAccessError;
use crate::repositories_traits::TrackInfoRepository;
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use models::{Car, Document, Role, TrackInfo, User};
use sqlx::{postgres::PgPoolOptions, PgPool, QueryBuilder, Row};

pub struct PgTrackInfoRepo {
    pool: PgPool,
}

impl PgTrackInfoRepo {
    pub async fn from(pg_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to PostgreSQL database for TrackInfo repository");
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
        Ok(PgTrackInfoRepo { pool })
    }
}

impl PgTrackInfoRepo {
    fn transform_mask_for_psql_like(gos_number: &str) -> String {
        log::debug!("Transforming gos number mask: {}", gos_number);
        let mut transformed_mask = gos_number.replace("*", "_");
        if let Some(last_char) = gos_number.chars().last() {
            if !last_char.is_digit(10) {
                transformed_mask.pop();
                transformed_mask.push('%');
            }
        }
        log::debug!("Transformed mask: {}", transformed_mask);
        transformed_mask
    }

    fn joined_tables_query() -> String {
        log::debug!("Generating joined tables query for track info");
        "SELECT
            t.route_date, 
            t.track_time, 
            a.name, 
            a.surname, 
            a.lastname, 
            a.login, 
            a.is_verified, 
            a.passport_serial, 
            a.passport_num, 
            a.role,
            o.name as oname, 
            o.surname as osurname, 
            o.lastname as olastname, 
            s.gos_num, 
            s.model, 
            s.mark,
            c.color, 
            s.release_date, 
            s.vin,
            s.sts_serial, 
            s.sts_num, 
            p.pts_serial, 
            p.pts_number
         FROM CarOwner o
         JOIN Car c ON o.id = c.owner_id
         JOIN TrackInfo t ON t.car_id = c.id
         JOIN AppUser a ON a.id = t.user_id
         JOIN STS s ON c.id = s.car_id
         JOIN PTS p ON c.id = p.id"
            .to_string()
    }

    fn form_rows_to_track_infos(rows: &[sqlx::postgres::PgRow]) -> Vec<TrackInfo> {
        log::debug!(
            "Converting {} database rows to TrackInfo objects",
            rows.len()
        );
        rows.into_iter()
            .map(|row| {
                let passport_serial: Option<i32> = row.get("passport_serial");
                let passport_num: Option<i32> = row.get("passport_num");

                let track_info = TrackInfo {
                    track_time: row
                        .get::<NaiveDateTime, _>("track_time")
                        .format("%H:%M %d.%m.%Y")
                        .to_string(),
                    route_date: row
                        .get::<NaiveDate, _>("route_date")
                        .format("%d.%m.%Y")
                        .to_string(),
                    car: Car {
                        gos_num: row.get("gos_num"),
                        model: row.get("model"),
                        owner_fio: (row.get("osurname"), row.get("oname"), row.get("olastname")),
                        mark: row.get("mark"),
                        color: row.get("color"),
                        year: row.get::<NaiveDate, _>("release_date").year() as u16,
                        vin: row.get("vin"),
                        sts: Document {
                            serial: format!("{:0>4}", row.get::<i32, _>("sts_serial").to_string()),
                            number: format!("{:0>6}", row.get::<i32, _>("sts_num").to_string()),
                        },
                        pts: Document {
                            serial: format!("{:0>4}", row.get::<i32, _>("pts_serial").to_string()),
                            number: format!("{:0>6}", row.get::<i32, _>("pts_number").to_string()),
                        },
                    },
                    user: User {
                        name: row.get("name"),
                        surname: row.get("surname"),
                        lastname: row.get("lastname"),
                        email: row.get("login"),
                        role: match row.get::<String, _>("role").as_str() {
                            "user" => Role::user,
                            "operator" => Role::operator,
                            "audit" => Role::audit,
                            _ => Role::user,
                        },
                        is_verified: row.get("is_verified"),
                        passport: match (passport_serial, passport_num) {
                            (Some(serial), Some(num)) => Some(Document {
                                serial: format!("{:0>4}", serial.to_string()),
                                number: format!("{:0>6}", num.to_string()),
                            }),
                            _ => None,
                        },
                    },
                };
                log::debug!("Converted TrackInfo: {:?}", track_info);
                track_info
            })
            .collect()
    }
}

#[async_trait]
impl TrackInfoRepository for PgTrackInfoRepo {
    async fn insert_track_info(
        &self,
        gos_num: &str,
        user_login: &str,
        route_date: &str,
    ) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting track info for vehicle {} by user {} on date {}",
            gos_num,
            user_login,
            route_date
        );

        let date = NaiveDate::parse_from_str(route_date, "%d.%m.%Y").map_err(|e| {
            log::error!("Invalid date format: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let moscow_time = Utc::now() + chrono::Duration::hours(3);
        let moscow_naive = moscow_time.naive_utc();
        log::debug!("Current Moscow time: {}", moscow_naive);

        let query = "
            INSERT INTO TrackInfo (car_id, user_id, route_date, track_time)
            SELECT 
                c.id, 
                a.id, 
                $1, 
                $2
            FROM 
                Car c
            JOIN 
                STS s ON s.car_id = c.id
            JOIN 
                AppUser a ON a.login = $3
            WHERE 
                s.gos_num = $4
            LIMIT 1;
        ";
        log::debug!("Executing insert query: {}", query);

        sqlx::query(query)
            .bind(date)
            .bind(moscow_naive)
            .bind(user_login)
            .bind(gos_num)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to insert track info: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Successfully inserted track info");
        Ok(())
    }

    async fn get_tracks_info_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        let transformed_gos_num = gos_num_mask.map(|gsn| Self::transform_mask_for_psql_like(gsn));
        log::info!(
            "Searching track info records by filters: {:?} {:?} {:?} {:?} {:?} {:?}",
            firstname,
            surname,
            lastname,
            passport,
            transformed_gos_num,
            date,
        );

        gos_num_mask.map(|gsn| Self::transform_mask_for_psql_like(gsn));

        let query = "SELECT * FROM get_tracks_info($1, $2, $3, $4, $5, $6, $7)";
        log::debug!("Executing query: {}", query);

        let sdate = match &date {
            Some(dt) => Some(NaiveDate::parse_from_str(dt, "%d.%m.%Y").map_err(|e| {
                log::error!("Invalid date format: {}", e);
                DataAccessError::InvalidInput(e.to_string())
            })?),
            None => None,
        };

        let pserial = match &passport {
            Some(psprt) => Some(psprt.serial.clone().parse::<i32>().map_err(|_| {
                log::error!("Invalid passport serial format: {}", &psprt.serial);
                DataAccessError::InvalidInput("Invalid passport serial format".to_string())
            })?),
            None => None,
        };

        let pnumber = match &passport {
            Some(psprt) => Some(psprt.number.clone().parse::<i32>().map_err(|_| {
                log::error!("Invalid passport number format: {}", &psprt.number);
                DataAccessError::InvalidInput("Invalid passport number format".to_string())
            })?),
            None => None,
        };

        let rows = sqlx::query(query)
            .bind(firstname)
            .bind(surname)
            .bind(lastname)
            .bind(sdate)
            .bind(transformed_gos_num)
            .bind(pserial)
            .bind(pnumber)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Found {} track info records by filters", rows.len());
        Ok(Self::form_rows_to_track_infos(&rows))
    }

    async fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!("Getting track info by date: {}", date);

        let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").map_err(|e| {
            log::error!("Invalid date format: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let where_query = "WHERE t.track_time::date = $1";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!("Executing query: {}", query);

        let rows = sqlx::query(query)
            .bind(date)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed for date {}: {}", date, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Found {} track info records for date {}", rows.len(), date);
        Ok(Self::form_rows_to_track_infos(&rows))
    }

    async fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!("Getting track info by gos number mask: {}", gos_number);

        let transformed_mask = Self::transform_mask_for_psql_like(gos_number);
        let where_query = "WHERE s.gos_num LIKE $1";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!(
            "Executing query: {} with param: {}",
            query,
            transformed_mask
        );

        let rows = sqlx::query(query)
            .bind(transformed_mask)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed for gos number mask {}: {}", gos_number, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!(
            "Found {} track info records matching mask {}",
            rows.len(),
            gos_number
        );
        Ok(Self::form_rows_to_track_infos(&rows))
    }

    async fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!(
            "Getting track info by user FIO: {:?} {:?} {:?}",
            name,
            surname,
            last_name
        );

        let mut query_builder = QueryBuilder::new(Self::joined_tables_query());

        if name.is_some() || surname.is_some() || last_name.is_some() {
            query_builder.push(" WHERE ");

            let mut needs_and = false;

            if let Some(name) = name {
                if needs_and {
                    query_builder.push(" AND ");
                }
                query_builder.push("a.name = ");
                query_builder.push_bind(name);
                needs_and = true;
            }

            if let Some(surname) = surname {
                if needs_and {
                    query_builder.push(" AND ");
                }
                query_builder.push("a.surname = ");
                query_builder.push_bind(surname);
                needs_and = true;
            }

            if let Some(last_name) = last_name {
                if needs_and {
                    query_builder.push(" AND ");
                }
                query_builder.push("a.lastname = ");
                query_builder.push_bind(last_name);
            }
        }

        let query = query_builder.build();

        let rows = query.fetch_all(&self.pool).await.map_err(|e| {
            log::error!("Query failed for FIO search: {}", e);
            DataAccessError::PsqlDataBaseError(e)
        })?;

        log::info!(
            "Found {} track info records matching FIO criteria",
            rows.len()
        );
        Ok(Self::form_rows_to_track_infos(&rows))
    }

    async fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!(
            "Getting track info by user passport: {}/{}",
            passport.serial,
            passport.number
        );

        let pserial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport serial format: {}", passport.serial);
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let pnum = passport.number.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport number format: {}", passport.number);
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;

        let where_query = "WHERE a.passport_serial = $1 AND a.passport_num = $2";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!(
            "Executing query: {} with params: {}, {}",
            query,
            pserial,
            pnum
        );

        let rows = sqlx::query(query)
            .bind(pserial)
            .bind(pnum)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed for passport search: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!(
            "Found {} track info records matching passport criteria",
            rows.len()
        );
        Ok(Self::form_rows_to_track_infos(&rows))
    }
}
