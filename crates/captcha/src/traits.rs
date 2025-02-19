use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Captcha {
    pub id: String,
    pub challenge: String,
    pub criteria: Option<String>,
}

impl Captcha {
    pub fn desensitize(self) -> Self {
        Self {
            criteria: None,
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Answer {
    pub id: Option<String>,
    pub content: String,
    pub client_ip: Option<String>,
}

#[derive(Debug, Error)]
pub enum CaptchaError {
    #[error("gone")]
    Gone,
    #[error("missing field: {0}")]
    MissingField(String),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}
