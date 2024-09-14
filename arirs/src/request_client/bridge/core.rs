use crate::*;

impl RequestClient {
    pub async fn bridge_create(&self, _bridge_id: &str) -> RequestClientResult<Bridge> {
        unimplemented!()
    }

    // SUGGESTION(gibbz00): combine with bidge_create by making ID optional
    pub async fn bridge_create_with_id(&self, _bridge_id: &str, _bridge: &Bridge) -> RequestClientResult<Bridge> {
        unimplemented!()
    }

    pub async fn bridge_get(_client: &RequestClient, _bridge_id: &str) -> RequestClientResult<Bridge> {
        unimplemented!()
    }

    pub async fn bidge_list(&self) -> RequestClientResult<Vec<Bridge>> {
        unimplemented!()
    }

    pub async fn bridge_destroy(&self, _bridge_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_add_channel(&self, _bridge_id: &str, _channel_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_remove_channel(&self, _bridge_id: &str, _channel_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_set_channel_as_video_source(
        &self,
        _bridge_id: &str,
        _channel_id: &str,
        _video_source_id: &str,
    ) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_unset_video_source(&self, _bridge_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_start_moh(&self, _bridge_id: &str, _moh_class: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_stop_moh(&self, _bridge_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_play_media(&self, _bridge_id: &str, _playback: &Playback) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_stop_media(&self, _bridge_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn bridge_start_recording(&self, _bridge_id: &str, _recording: &LiveRecording) -> RequestClientResult<()> {
        unimplemented!()
    }
}
