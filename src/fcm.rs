mod secrets;
mod token_manager;
mod waiter;

use std::time::Duration;
use serde::Serialize;
use serde_json::json;
use tokio::time::timeout;
pub use secrets::*;
use token_manager::*;
use crate::error::FcmError;
use crate::request::FcmRequest;

pub struct FCM {
    project_id: String,
    manager: TokenManager,
}

impl FCM {
    pub async fn new(project_id: impl ToString, secrets: Secrets) -> Result<Self, FcmError> {
        let manager = TokenManager::new(secrets).await?;
        Ok(Self {
            project_id: project_id.to_string(),
            manager,
        })
    }

    pub async fn send_message<'a, Data: Serialize>(&self, request: FcmRequest<'a, Data>) -> Result<(), FcmError> {
        use reqwest::*;

        let token = self.manager.load_token().await?;

        let response = Client::new()
            .post(format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                self.project_id
            ))
            .bearer_auth(token)
            .json(&json!({
                "message": request
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();

            let timeout = timeout(
                Duration::from_secs(1),
                response.text()
            ).await;

            let text = timeout
                .map_err(|_| "Timed-out when reading the response body!".to_string())
                .unwrap_or_else(|msg| Ok(msg))
                .unwrap_or_default();

            Err(FcmError::ServerError(status, text))
        } else {
            Ok(())
        }
    }
}
