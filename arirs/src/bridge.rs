use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub id: String,
}

impl Bridge {
    pub async fn destroy(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn add_channel(&self, _client: &RequestClient, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn remove_channel(&self, _client: &RequestClient, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn set_channel_as_video_source(&self, _client: &RequestClient, _channel_id: &str, _video_source_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn unset_video_source(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_moh(&self, _client: &RequestClient, _moh_class: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_moh(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn play_media(&self, _client: &RequestClient, _playback: &Playback) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_media(&self, _client: &RequestClient) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_recording(&self, _client: &RequestClient, _recording: &LiveRecording) -> Result<()> {
        unimplemented!()
    }

    pub async fn list_bridges(_client: &RequestClient) -> Result<Vec<Bridge>> {
        unimplemented!()
    }

    pub async fn create_bridge(_client: &RequestClient, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn create_bridge_with_id(_client: &RequestClient, _bridge_id: &str, _bridge: &Bridge) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn get_bridge(_client: &RequestClient, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }
}
