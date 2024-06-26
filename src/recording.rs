use crate::client::Client;
use crate::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiveRecording {
    pub id: String,
    pub name: String,
}

impl LiveRecording {
    #[instrument(level = "debug")]
    pub async fn get(_recording_name: &str) -> Result<LiveRecording> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn discard(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    // TODO: explore if it's possible to return a StoredRecording
    #[instrument(level = "debug")]
    pub async fn stop(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn pause(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn resume(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn mute(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn unmute(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StoredRecording {
    pub id: String,
    pub format: String,
}

impl StoredRecording {
    #[instrument(level = "debug")]
    pub async fn list(_client: &Client) -> Result<Vec<StoredRecording>> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn get(_recording_name: &str) -> Result<StoredRecording> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn delete(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument(level = "debug")]
    pub async fn download(&self, _client: &Client) -> Result<&[u8]> {
        unimplemented!()
    }
}
