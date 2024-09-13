use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiveRecording {
    pub id: String,
    pub name: String,
}

impl LiveRecording {
    pub async fn get(_recording_name: &str) -> Result<LiveRecording> {
        unimplemented!()
    }

    pub async fn discard(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    // TODO: explore if it's possible to return a StoredRecording
    pub async fn stop(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn pause(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn resume(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn mute(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn unmute(&self, _client: &RequestClient) -> Result<()> {
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
    pub async fn list(_client: &RequestClient) -> Result<Vec<StoredRecording>> {
        unimplemented!()
    }

    pub async fn get(_recording_name: &str) -> Result<StoredRecording> {
        unimplemented!()
    }

    pub async fn delete(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn download(&self, _client: &RequestClient) -> Result<&[u8]> {
        unimplemented!()
    }
}
