use serde::Serialize;

#[derive(Serialize)]
pub struct FcmNotification {
    pub title: String,
    pub body: String,
}
