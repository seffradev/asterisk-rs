use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LiveRecording {
    pub id: String,
    pub name: String,
}

impl RequestClient {
    pub async fn live_recording_get(&self, _recording_name: &str) -> Result<LiveRecording> {
        unimplemented!()
    }

    pub async fn live_recording_discard(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    // TODO: explore if it's possible to return a StoredRecording
    pub async fn live_recording_stop(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_pause(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_resume(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_mute(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn live_recording_unmute(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StoredRecording {
    pub id: String,
    pub format: String,
}

impl RequestClient {
    pub async fn stored_recording_list(&self, _recording_name: &str) -> Result<Vec<StoredRecording>> {
        unimplemented!()
    }

    pub async fn stored_recording_get(&self, _recording_name: &str) -> Result<StoredRecording> {
        unimplemented!()
    }

    pub async fn stored_recording_delete(&self, _recording_name: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn stored_recording_download(&self, _recording_name: &str) -> Result<&[u8]> {
        unimplemented!()
    }
}
