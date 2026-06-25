use sea_orm::DbErr;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepositoryError {
    DbError(DbErr),
    SerdeError(postcard::Error),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::DbError(error) => write!(f, "Database error: {error}"),
            RepositoryError::SerdeError(error) => write!(f, "Serde error: {error}"),
        }
    }
}

impl Error for RepositoryError {}

impl From<DbErr> for RepositoryError {
    fn from(value: DbErr) -> Self {
        Self::DbError(value)
    }
}

impl From<postcard::Error> for RepositoryError {
    fn from(value: postcard::Error) -> Self {
        Self::SerdeError(value)
    }
}
