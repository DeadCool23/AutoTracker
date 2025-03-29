#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    #[error(transparent)]
    PsqlDataBaseError(#[from] sqlx::Error),
    #[error(transparent)]
    RedisDataBaseError(#[from] redis::RedisError),
}
