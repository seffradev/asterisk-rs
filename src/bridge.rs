use crate::Result;
use crate::{client::Client, playback::Playback, recording::LiveRecording};
use serde::{Deserialize, Serialize};

impl Bridge {
    pub async fn destroy(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn add_channel(&self, _client: &Client, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn remove_channel(&self, _client: &Client, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn set_channel_as_video_source(
        &self,
        _client: &Client,
        _channel_id: &str,
        _video_source_id: &str,
    ) -> Result<()> {
        unimplemented!()
    }

    pub async fn unset_video_source(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_moh(&self, _client: &Client, _moh_class: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_moh(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn play_media(&self, _client: &Client, _playback: &Playback) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop_media(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn start_recording(&self, _client: &Client, _recording: &LiveRecording) -> Result<()> {
        unimplemented!()
    }
}

impl Client {
    pub async fn list_bridges(&self) -> Result<Vec<Bridge>> {
        unimplemented!()
    }

    pub async fn create_bridge(&self, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn create_bridge_with_id(
        &self,
        _bridge_id: &str,
        _bridge: &Bridge,
    ) -> Result<Bridge> {
        unimplemented!()
    }

    pub async fn get_bridge(&self, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub id: String,
}