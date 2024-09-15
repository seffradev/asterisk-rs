use crate::*;

impl AriClient {
    pub async fn playback_get(&self, _playback_id: &str) -> AriClientResult<Playback> {
        unimplemented!()
    }

    pub async fn playback_control(&self, _playback_id: &str, _operation: Operation) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn playback_stop(&self, _playback_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }
}
