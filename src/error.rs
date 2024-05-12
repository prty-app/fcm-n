// todo: Rework and document

#[derive(thiserror::Error, Debug)]
pub enum FcmError {
    #[error("Issue with request to Google: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Google server returned error: {0}, {1}")]
    ServerError(u16, String),
    #[error("Tried to parse invalid JSON: {0}")]
    InvalidJSON(#[from] serde_json::Error),
    #[error("Issue with JWT: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error)
}
