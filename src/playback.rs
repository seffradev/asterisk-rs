use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::Result;

impl Playback {
    pub async fn control(&self, _client: &Client, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }
}

impl Client {
    pub async fn get_playback(&self, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    pub id: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Restart,
    Pause,
    Unpause,
    Reverse,
    Forward,
}
