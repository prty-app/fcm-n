mod notification;

use serde::Serialize;
pub use notification::*;

#[derive(Serialize)]
pub struct FcmRequest<'a, Data: Serialize> {
    #[serde(rename="token")]
    pub device_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<FcmNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl<'a, Data: Serialize> FcmRequest<'a, Data> {
    pub fn new(device_token: &'a str) -> Self {
        Self {
            device_token,
            notification: None,
            data: None,
        }
    }

    pub fn set_notification(mut self, notification: FcmNotification) -> Self {
        self.notification = Some(notification);
        self
    }

    pub fn set_data(mut self, data: Data) -> Self {
        self.data = Some(data);
        self
    }
}
