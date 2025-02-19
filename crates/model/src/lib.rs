use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub filename: String,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
    pub accuracy: Option<f64>,
    pub note: Option<String>,
}
