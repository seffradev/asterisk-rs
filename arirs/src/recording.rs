use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiveRecording {
    pub id: String,
    pub name: String,
}

impl LiveRecording {
    pub async fn live_recording_get(_recording_name: &str) -> Result<LiveRecording> {
        unimplemented!()
    }

    pub async fn live_recording_discard(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    // TODO: explore if it's possible to return a StoredRecording
    pub async fn live_recording_stop(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_pause(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_resume(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_mute(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_unmute(&self, _client: &RequestClient) -> Result<()> {
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
    pub async fn stored_recording_list(_client: &RequestClient) -> Result<Vec<StoredRecording>> {
        unimplemented!()
    }

    pub async fn stored_recording_get(_recording_name: &str) -> Result<StoredRecording> {
        unimplemented!()
    }

    pub async fn stored_recording_delete(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn stored_recording_download(&self, _client: &RequestClient) -> Result<&[u8]> {
        unimplemented!()
    }
}
