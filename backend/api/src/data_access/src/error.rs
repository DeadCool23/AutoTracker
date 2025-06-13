#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    #[error("Can't reconnect")]
    ReconnectionError,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Not founded: {0}")]
    NotFoundError(String),
    #[error(transparent)]
    PsqlDataBaseError(#[from] sqlx::Error),
    #[error(transparent)]
    RedisDataBaseError(#[from] redis::RedisError),
    #[error(transparent)]
    ClickHouseBaseError(#[from] clickhouse::error::Error),
}
