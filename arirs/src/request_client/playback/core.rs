use crate::*;

impl RequestClient {
    pub async fn playback_get(&self, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub async fn playback_control(&self, _playback_id: &str, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn playback_stop(&self, _playback_id: &str) -> Result<()> {
        unimplemented!()
    }
}
