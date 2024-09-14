use crate::*;

impl RequestClient {
    pub async fn get_playback(&self, _playback_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub async fn control(&self, _playback_id: &str, _operation: Operation) -> Result<()> {
        unimplemented!()
    }

    pub async fn stop(&self, _playback_id: &str) -> Result<()> {
        unimplemented!()
    }
}
