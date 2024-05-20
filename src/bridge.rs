use crate::Result;
use crate::{client::Client, playback::Playback, recording::Recording};
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn list_bridges(&self) -> Result<Vec<Bridge>> {
        unimplemented!()
    }

    pub async fn create_bridge(&self, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn create_bridge_with_id(&self, _bridge_id: &str, _bridge: &Bridge) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn get_bridge(&self, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn destroy_bridge(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn add_channel_to_bridge(&self, _bridge_id: &str, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn remove_channel_from_bridge(
        &self,
        _bridge_id: &str,
        _channel_id: &str,
    ) -> Result<()> {
        unimplemented!()
    }

    pub async fn set_bridge_as_video_source(
        &self,
        _bridge_id: &str,
        _channel_id: &str,
        _video_source_id: &str,
    ) -> Result<()> {
        unimplemented!()
    }

    pub async fn unset_bridge_as_video_source(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_moh_bridge(&self, _bridge_id: &str, _moh_class: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_moh_bridge(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn play_bridge_media(&self, _bridge_id: &str, _playback: &Playback) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_bridge_media(&self, _bridge_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_bridge_recording(
        &self,
        _bridge_id: &str,
        _recording: &Recording,
    ) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub id: String,
}
