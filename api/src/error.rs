use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("room not found")]
    RoomNotFound,
    #[error("ticket type not found")]
    TicketTypeNotFound,
    #[error("ticket type inactive")]
    TicketTypeInactive,
    #[error("ticket not found")]
    TicketNotFound,
    #[error("invalid ticket transition: {0}")]
    InvalidTransition(&'static str),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::RoomNotFound | Self::TicketNotFound | Self::TicketTypeNotFound => {
                StatusCode::NOT_FOUND
            }
            Self::TicketTypeInactive => StatusCode::BAD_REQUEST,
            Self::InvalidTransition(_) => StatusCode::CONFLICT,
        }
    }
}
