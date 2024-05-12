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

/// FCM context object.
/// Takes care of everything needed to send FCM notifications.
///
/// To create you will have to have your FCM project id and the private JSON credentials file.
///
/// ### Important
/// Do not create the context object each time you have to send a notification.
/// Pass it in some app state object, or allocate it globally.
/// This structure doesn't require mutability, so you don't have to use any locks.
///
/// If you do create more instances of the `FCM` structure each will have its own unique
/// token manager which would cause more requests to the FCM API than it's necessary.
pub struct FCM {
    project_id: String,
    manager: TokenManager,
}

impl FCM {
    /// Creates new FCM context object.
    ///
    /// # Arguments
    /// - `project_id` - ID of your FCM project.
    /// - `secrets` - Object with required fields from the private JSON credentials file.
    ///
    /// # Errors
    /// Will result in an error if the token cannot be created.
    /// This would occur if there is no internet connection, wrong arguments are passed, or some
    /// other issue with the FCM API.
    pub async fn new(project_id: impl ToString, secrets: Secrets) -> Result<Self, FcmError> {
        let manager = TokenManager::new(secrets).await?;
        Ok(Self {
            project_id: project_id.to_string(),
            manager,
        })
    }

    /// Sends a new FCM message to show a notification on a specific device.
    ///
    /// # Arguments
    /// - `request` - Object that contains the notification settings.
    ///
    /// # Errors
    /// Will result in an error if the token cannot be refreshed (if expired), or for any other
    /// FCM API issue.
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
