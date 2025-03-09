use async_graphql::Error;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    DatabaseError(String),
}

impl From<sqlx::Error> for UserError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => UserError::NotFound,
            _ => UserError::DatabaseError(error.to_string())
        }
    }
}

impl From<UserError> for async_graphql::Error {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound => Error::new("User not found"),
            UserError::DatabaseError(msg) => Error::new(format!("Database error: {}", msg)),
        }
    }
}

