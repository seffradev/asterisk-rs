use crate::client::Client;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    pub id: String,
}

impl Playback {
    pub async fn get_playback(_client: &Client, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub async fn control(&self, _client: &Client, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop(&self, _client: &Client) -> Result<()> {
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

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Restart => write!(f, "restart"),
            Operation::Pause => write!(f, "pause"),
            Operation::Unpause => write!(f, "unpause"),
            Operation::Reverse => write!(f, "reverse"),
            Operation::Forward => write!(f, "forward"),
        }
    }
}
