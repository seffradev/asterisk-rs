use thiserror::Error;

pub type Result<T> = std::result::Result<T, AriError>;

#[derive(Debug, Error)]
pub enum AriError {
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error("HTTP Request error")]
    Reqwest(#[from] reqwest::Error),
}
