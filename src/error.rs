use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Request Error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("URL Error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Zip Error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Generic Error: {0}")]
    Generic(String),
}
