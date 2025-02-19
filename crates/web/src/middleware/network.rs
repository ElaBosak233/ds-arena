use axum::extract::Request;
use serde::{Deserialize, Serialize};
use tower_governor::{GovernorError, key_extractor::KeyExtractor};

use crate::util::network::get_client_ip;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct GovernorKeyExtractor;

impl KeyExtractor for GovernorKeyExtractor {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        let ip = get_client_ip(req).ok_or(GovernorError::UnableToExtractKey)?;
        Ok(ip.to_string())
    }
}
