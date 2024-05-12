mod notification;

use serde::Serialize;
pub use notification::*;

/// Builder for the FCM message request used to create a notification.
///
/// `Data` is the generic type you provide when you need additional custom fields.
/// If you don't need that and provide a `None` variant on the `data` field the compiler
/// could require a type either way, when that happens just use `()` as the type.
#[derive(Serialize)]
pub struct FcmRequest<'a, Data: Serialize> {
    /// Token of the device the notification should be sent to.
    #[serde(rename="token")]
    pub device_token: &'a str,
    /// Basic notification fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<FcmNotification>,
    /// Custom notification fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl<'a, Data: Serialize> FcmRequest<'a, Data> {
    /// Creates a new FCM message request with no fields.
    ///
    /// ### Arguments
    /// - `device_token` - Token of the device the notification should be sent to.
    ///
    /// ### Important
    /// Sending empty notification might result in an error!
    pub fn new(device_token: &'a str) -> Self {
        Self {
            device_token,
            notification: None,
            data: None,
        }
    }

    /// Sets the basic notification fields.
    ///
    /// Will override any previous change to this field.
    pub fn set_notification(mut self, notification: FcmNotification) -> Self {
        self.notification = Some(notification);
        self
    }

    /// Sets the custom notification fields.
    ///
    /// Will override any previous change to this field.
    pub fn set_data(mut self, data: Data) -> Self {
        self.data = Some(data);
        self
    }
}
