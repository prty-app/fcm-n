use std::io::*;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Secrets {
    pub private_key: String,
    pub client_email: String,
    pub private_key_id: String,
}

impl Secrets {
    pub async fn new(
        private_key: impl ToString,
        client_email: impl ToString,
        private_key_id: impl ToString,
    ) -> Self {
        Self {
            private_key: private_key.to_string(),
            client_email: client_email.to_string(),
            private_key_id: private_key_id.to_string(),
        }
    }

    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = tokio::fs::read_to_string(path).await?;
        serde_json::from_str(&contents).map_err(|err| Error::new(
            ErrorKind::InvalidData,
            err,
        ))
    }
}
