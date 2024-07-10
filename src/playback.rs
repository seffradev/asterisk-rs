use derive_more::Display;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{client::Client, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    pub id: String,
}

impl Playback {
    #[instrument(level = "debug")]
    pub async fn get_playback(_client: &Client, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn control(&self, _client: &Client, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn stop(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug, Display)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    #[display(fmt = "restart")]
    Restart,
    #[display(fmt = "pause")]
    Pause,
    #[display(fmt = "unpause")]
    Unpause,
    #[display(fmt = "reverse")]
    Reverse,
    #[display(fmt = "forward")]
    Forward,
}
