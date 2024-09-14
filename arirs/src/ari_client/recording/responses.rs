use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct LiveRecording {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StoredRecording {
    id: String,
    format: String,
}
