use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub type Result<T> = std::result::Result<T, AriError>;

#[derive(Debug, Error)]
pub enum AriError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error("WebSocket error")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("HTTP Request error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}

impl From<tungstenite::error::UrlError> for AriError {
    fn from(err: tungstenite::error::UrlError) -> Self {
        AriError::Tungstenite(err.into())
    }
}
