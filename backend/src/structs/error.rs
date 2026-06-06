use actix_session::SessionInsertError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to connect to persistent database")]
    PersistentDatabase(#[from] sqlx::Error),

    #[error("failed to connect to ActixSession")]
    Session(#[from] SessionInsertError),
}
