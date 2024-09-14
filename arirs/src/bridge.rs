use derive_getters::Getters;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize, Getters)]
pub struct Bridge {
    pub id: String,
}

impl RequestClient {
    pub async fn destroy(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn add_channel(&self, _bridge_id: &str, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn remove_channel(&self, _bridge_id: &str, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn set_channel_as_video_source(&self, _bridge_id: &str, _channel_id: &str, _video_source_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn unset_video_source(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn bridge_start_moh(&self, _bridge_id: &str, _moh_class: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn bridge_stop_moh(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn bridge_play_media(&self, _bridge_id: &str, _playback: &Playback) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_media(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_recording(&self, _bridge_id: &str, _recording: &LiveRecording) -> Result<()> {
        unimplemented!()
    }

    pub async fn list_bridges(&self) -> Result<Vec<Bridge>> {
        unimplemented!()
    }

    pub async fn create_bridge(&self, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn create_bridge_with_id(&self, _bridge_id: &str, _bridge: &Bridge) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn get_bridge(_client: &RequestClient, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }
}
