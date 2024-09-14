use crate::*;

impl RequestClient {
    pub async fn playback_get(&self, _playback_id: &str) -> RequestClientResult<Playback> {
        unimplemented!()
    }

    pub async fn playback_control(&self, _playback_id: &str, _operation: Operation) -> RequestClientResult<()> {
        unimplemented!()
    }

    pub async fn playback_stop(&self, _playback_id: &str) -> RequestClientResult<()> {
        unimplemented!()
    }
}
