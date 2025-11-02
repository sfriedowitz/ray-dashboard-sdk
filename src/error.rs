use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Request Error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("URL Error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Generic Error: {0}")]
    Generic(String),
}
