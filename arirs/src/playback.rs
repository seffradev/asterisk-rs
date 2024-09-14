use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    pub id: String,
}

impl Playback {
    pub async fn get_playback(_client: &RequestClient, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub async fn control(&self, _client: &RequestClient, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Restart,
    Pause,
    Unpause,
    Reverse,
    Forward,
}
