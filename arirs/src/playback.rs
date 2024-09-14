use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    pub id: String,
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

impl RequestClient {
    pub async fn get_playback(&self, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub async fn control(&self, _playback_id: &str, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop(&self, _playback_id: &str) -> Result<()> {
        unimplemented!()
    }
}
