use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    id: String,
}
