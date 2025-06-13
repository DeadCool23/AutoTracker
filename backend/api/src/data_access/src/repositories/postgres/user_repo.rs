use crate::error::DataAccessError;
use crate::repositories_traits::UserRepository;
use async_trait::async_trait;
use models::{Document, Role, User};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub async fn from(pg_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to PostgreSQL database for User repository");
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
        Ok(PgUserRepo { pool })
    }
}

impl PgUserRepo {
    fn form_row_to_user(row: &PgRow) -> User {
        log::debug!("Converting database row to User");
        let passport_serial: Option<i32> = row.get("passport_serial");
        let passport_num: Option<i32> = row.get("passport_num");

        let user = User {
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
        };
        log::debug!("Converted user: {:?}", user);
        user
    }
}

#[async_trait]
impl UserRepository for PgUserRepo {
    async fn get_user_by_auth_info(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, DataAccessError> {
        log::info!("Authenticating user with email: {}", email);
        let query = "SELECT * FROM AppUser WHERE login = $1 AND password = $2 LIMIT 1";
        log::debug!("Executing auth query: {}", query);

        let row = sqlx::query(query)
            .bind(email)
            .bind(password)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Authentication query failed for {}: {}", email, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        match row {
            Some(row) => {
                log::info!("User authenticated successfully: {}", email);
                Ok(Some(Self::form_row_to_user(&row)))
            }
            None => {
                log::warn!("Authentication failed for user: {}", email);
                Ok(None)
            }
        }
    }

    async fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<User>, DataAccessError> {
        log::info!("Getting user by passport: {:#?}", passport);
        let serial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport serial format: {}", passport.serial);
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let number = passport.number.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport number format: {}", passport.number);
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;
        let query =
            "SELECT * FROM AppUser WHERE passport_serial = $1 AND passport_num = $2 LIMIT 1";
        log::debug!("Executing query: {}", query);

        let row = sqlx::query(query)
            .bind(serial)
            .bind(number)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed for email {:#?}: {}", passport, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        match row {
            Some(row) => {
                log::info!("User found: {:#?}", passport);
                Ok(Some(Self::form_row_to_user(&row)))
            }
            None => {
                log::info!("User not found: {:#?}", passport);
                Ok(None)
            }
        }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError> {
        log::info!("Getting user by email: {}", email);
        let query = "SELECT * FROM AppUser WHERE login = $1 LIMIT 1";
        log::debug!("Executing query: {}", query);

        let row = sqlx::query(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Query failed for email {}: {}", email, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        match row {
            Some(row) => {
                log::info!("User found: {}", email);
                Ok(Some(Self::form_row_to_user(&row)))
            }
            None => {
                log::info!("User not found: {}", email);
                Ok(None)
            }
        }
    }

    async fn insert_user(&self, user: &User, password: &str) -> Result<(), DataAccessError> {
        log::info!("Inserting new user: {}", user.email);
        let (passport_serial, passport_num) = match &user.passport {
            Some(passport) => {
                log::debug!("Parsing passport data for user: {}", user.email);
                (
                    Some(passport.serial.parse::<i32>().map_err(|_| {
                        log::error!("Invalid passport serial format for user: {}", user.email);
                        DataAccessError::InvalidInput("Invalid passport serial format".to_string())
                    })?),
                    Some(passport.number.parse::<i32>().map_err(|_| {
                        log::error!("Invalid passport number format for user: {}", user.email);
                        DataAccessError::InvalidInput("Invalid passport number format".to_string())
                    })?),
                )
            }
            None => (None, None),
        };

        let query = "INSERT INTO AppUser (
            login, password, role, name, surname, lastname, 
            is_verified, passport_serial, passport_num
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
        log::debug!("Executing insert query: {}", query);

        sqlx::query(query)
            .bind(&user.email)
            .bind(password)
            .bind(match user.role {
                Role::user => "user",
                Role::operator => "operator",
                Role::audit => "audit",
            })
            .bind(&user.name)
            .bind(&user.surname)
            .bind(&user.lastname)
            .bind(user.is_verified)
            .bind(passport_serial)
            .bind(passport_num)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to insert user {}: {}", user.email, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("User inserted successfully: {}", user.email);
        Ok(())
    }

    async fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError> {
        log::info!("Updating passport for user: {}", email);
        let serial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!(
                "Invalid passport serial format for user {}: {}",
                email,
                passport.serial
            );
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let number = passport.number.parse::<i32>().map_err(|_| {
            log::error!(
                "Invalid passport number format for user {}: {}",
                email,
                passport.number
            );
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;

        let query = "CALL verify_user($1, $2, $3)";
        log::debug!("Executing stored procedure: {}", query);

        sqlx::query(query)
            .bind(email)
            .bind(serial)
            .bind(number)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to update passport for user {}: {}", email, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Passport updated successfully for user: {}", email);
        Ok(())
    }
}

impl PgUserRepo {
    pub async fn delete_user(&self, user: &User) -> Result<(), DataAccessError> {
        log::info!("Deleting user: {}", user.email);
        let query = "DELETE FROM AppUser WHERE login = $1";
        log::debug!("Executing delete query: {}", query);

        sqlx::query(query)
            .bind(&user.email)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to delete user {}: {}", user.email, e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("User deleted successfully: {}", user.email);
        Ok(())
    }
}
