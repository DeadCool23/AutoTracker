use crate::error::DataAccessError;
use crate::repositories_traits::CarRepository;
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate};
use models::{Car, Document};
use sqlx::{postgres::PgPoolOptions, PgPool, QueryBuilder, Row};

pub struct PgCarRepo {
    pool: PgPool,
}

impl PgCarRepo {
    pub async fn from(pg_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to PostgreSQL database for Car repository");
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
        Ok(PgCarRepo { pool })
    }
}

impl PgCarRepo {
    fn transform_mask_for_psql_like(gos_number: &str) -> String {
        log::debug!("Transforming mask for PostgreSQL LIKE: {}", gos_number);
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
        log::debug!("Generating joined tables query for cars");
        "SELECT 
            o.name, 
            o.surname, 
            o.lastname, 
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
         JOIN STS s ON c.id = s.car_id
         JOIN PTS p ON c.id = p.id"
            .to_string()
    }

    fn form_rows_to_cars(rows: &[sqlx::postgres::PgRow]) -> Vec<Car> {
        log::debug!("Converting {} database rows to Car objects", rows.len());
        rows.into_iter()
            .map(|row| {
                let car = Car {
                    gos_num: row.get("gos_num"),
                    model: row.get("model"),
                    owner_fio: (row.get("surname"), row.get("name"), row.get("lastname")),
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
                };
                log::debug!("Converted car: {:?}", car);
                car
            })
            .collect()
    }
}

#[async_trait]
impl CarRepository for PgCarRepo {
    async fn get_cars_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        let transformed_gos_num = gos_num_mask.map(|gsn| Self::transform_mask_for_psql_like(gsn));
        log::info!(
            "Searching cars by filters: {:?} {:?} {:?} {:?} {:?}",
            firstname,
            surname,
            lastname,
            transformed_gos_num,
            passport,
        );

        let query = "SELECT * FROM get_cars($1, $2, $3, $4, $5, $6)";
        log::debug!("Executing query: {}", query);

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
            .bind(transformed_gos_num)
            .bind(pserial)
            .bind(pnumber)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Found {} cars matching filters", rows.len());
        Ok(Self::form_rows_to_cars(&rows))
    }

    async fn get_car_by_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<Car>, DataAccessError> {
        log::info!("Searching cars by gos number mask: {}", gos_number);
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

        log::info!("Found {} cars matching mask {}", rows.len(), gos_number);
        Ok(Self::form_rows_to_cars(&rows))
    }

    async fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        log::info!(
            "Searching cars by owner FIO: {:?} {:?} {:?}",
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
                query_builder.push("o.name = ");
                query_builder.push_bind(name);
                needs_and = true;
            }

            if let Some(surname) = surname {
                if needs_and {
                    query_builder.push(" AND ");
                }
                query_builder.push("o.surname = ");
                query_builder.push_bind(surname);
                needs_and = true;
            }

            if let Some(last_name) = last_name {
                if needs_and {
                    query_builder.push(" AND ");
                }
                query_builder.push("o.lastname = ");
                query_builder.push_bind(last_name);
            }
        }

        let query = query_builder.build();

        let rows = query.fetch_all(&self.pool).await.map_err(|e| {
            log::error!("Query failed for FIO search: {}", e);
            DataAccessError::PsqlDataBaseError(e)
        })?;

        log::info!("Found {} cars matching FIO criteria", rows.len());
        Ok(Self::form_rows_to_cars(&rows))
    }

    async fn get_car_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, DataAccessError> {
        log::info!(
            "Searching cars by owner passport: {}/{}",
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

        let where_query = "WHERE o.passport_serial = $1 AND o.passport_num = $2";
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

        log::info!("Found {} cars matching passport criteria", rows.len());
        Ok(Self::form_rows_to_cars(&rows))
    }
}
