use data_access::error::DataAccessError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    DataAccessError(#[from] DataAccessError),
    #[error("Invalid data: {0}")]
    InvalidDataError(String),
    #[error("{0} already exist")]
    IsExistError(String),
    #[error("{0} is not found")]
    NotFoundError(String),
}
