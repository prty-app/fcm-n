use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::RwLock;
use crate::error::FcmError;
use crate::fcm::Secrets;
use crate::fcm::waiter::Waiter;

static REFRESH_JWT_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
static REFRESH_JWT_AUDIENCE: &str = "https://oauth2.googleapis.com/token";

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Clock gone backwards! System time is set before UNIX EPOCH!")
        .as_secs()
}

pub struct TokenManager {
    secrets: Secrets,
    token: RwLock<Arc<String>>,
    exp: AtomicU64,
    waiter: Waiter,
}

impl TokenManager {
    pub async fn new(secrets: Secrets) -> Result<Self, FcmError> {
        let this = Self {
            secrets,
            token: RwLock::new(Arc::new(String::new())),
            exp: AtomicU64::new(0),
            waiter: Waiter::new(),
        };

        this.refresh().await?;

        Ok(this)
    }

    pub async fn refresh(&self) -> Result<(), FcmError> {
        use reqwest::*;

        #[derive(Deserialize)]
        struct RefreshResponse {
            access_token: String,
            expires_in: u64,
        }

        let refresh_jwt = self.create_refresh_jwt()?;

        let response = Client::new()
            .post(REFRESH_JWT_AUDIENCE)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &refresh_jwt),
            ])
            .send()
            .await?
            .json::<RefreshResponse>()
            .await?;

        self.update_token(
            response.access_token,
            now() + response.expires_in,
        ).await;

        Ok(())
    }

    pub async fn load_token(&self) -> Result<Arc<String>, FcmError> {
        if self.is_expired() {
            if self.waiter.is_waiting() {
                self.waiter.wait().await;
            } else {
                self.waiter.start();
                self.refresh().await?;
                self.waiter.stop();
            }
        }

        let token = self.token.read().await.clone();
        Ok(token)
    }

    async fn update_token(&self, token: String, exp: u64) {
        let mut guard = self.token.write().await;
        *guard = Arc::new(token);
        self.exp.store(exp, Ordering::SeqCst);
    }

    pub fn is_expired(&self) -> bool {
        self.exp.load(Ordering::SeqCst) <= now()
    }

    fn create_refresh_jwt(&self) -> Result<String, FcmError> {
        use jsonwebtoken::*;

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.secrets.private_key.clone());

        let now_ts = now();

        let claims = json!({
            "iss": self.secrets.client_email,
            "scope": REFRESH_JWT_SCOPE,
            "aud": REFRESH_JWT_AUDIENCE,
            "exp": now_ts + 3600,
            "iat": now_ts
        });

        let encoding_key = EncodingKey::from_rsa_pem(self.secrets.private_key.as_bytes())?;
        let signed_jwt = encode(&header, &claims, &encoding_key)?;
        Ok(signed_jwt)
    }
}
