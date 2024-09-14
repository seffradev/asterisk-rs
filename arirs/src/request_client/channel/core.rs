use std::collections::HashMap;

use crate::*;

impl RequestClient {
    pub async fn answer(&self, channel_id: &str) -> Result<()> {
        self.authorized_post(["channels", channel_id, "answer"], ()).await
    }

    pub async fn hangup(&self, channel_id: &str, reason: Reason) -> Result<()> {
        self.authorized_delete(["channels", channel_id], reason).await
    }

    pub async fn start_ringing(&self, channel_id: &str) -> Result<()> {
        self.authorized_post(["channels", channel_id, "ring"], ()).await
    }

    pub async fn stop_ringing(&self, channel_id: &str) -> Result<()> {
        self.authorized_delete(["channels", channel_id, "ring"], ()).await
    }

    pub async fn send_dtmf(&self, channel_id: &str, params: SendDtmfParams<'_>) -> Result<()> {
        self.authorized_post(["channels", channel_id, "dtmf"], params).await
    }

    pub async fn mute(&self, channel_id: &str, direction: Direction) -> Result<()> {
        self.authorized_post(["channels", channel_id, "mute"], direction).await
    }

    pub async fn unmute(&self, channel_id: &str, direction: Direction) -> Result<()> {
        self.authorized_delete(["channels", channel_id, "mute"], direction).await
    }

    pub async fn hold(&self, channel_id: &str) -> Result<()> {
        self.authorized_post(["channels", channel_id, "hold"], ()).await
    }

    pub async fn unhold(&self, channel_id: &str) -> Result<()> {
        self.authorized_delete(["channels", channel_id, "hold"], ()).await
    }

    pub async fn play_media(&self, channel_id: &str, params: PlayMediaParams<'_>) -> Result<Playback> {
        self.authorized_post_json_response(["channels", channel_id, "play"], params).await
    }

    pub async fn play_media_with_id(&self, channel_id: &str, playback_id: &str, params: PlayMediaBaseParams<'_>) -> Result<Playback> {
        self.authorized_post_json_response(["channels", channel_id, "play", playback_id, "media"], params)
            .await
    }

    pub async fn record(&self, channel_id: &str, params: RecordParams<'_>) -> Result<LiveRecording> {
        self.authorized_post_json_response(["channels", channel_id, "record"], params).await
    }

    pub async fn dial(&self, channel_id: &str, params: DialParams<'_>) -> Result<()> {
        self.authorized_post(["channels", channel_id, "dial"], params).await
    }

    pub async fn list(&self) -> Result<Vec<Channel>> {
        self.authorized_get(["channels"], ()).await
    }

    pub async fn create(&self, params: ChannelCreateParams<'_>, variables: &HashMap<&str, &str>) -> Result<Channel> {
        self.authorized_post_variables(["channels", "create"], params, variables).await
    }

    pub async fn get(self, channel_id: &str) -> Result<Channel> {
        self.authorized_get(["channels", channel_id], ()).await
    }

    pub async fn originate<'a>(&self, params: OriginateChannelParams<'a>, variables: &HashMap<&str, &str>) -> Result<Channel> {
        self.authorized_post_variables(["channels"], params, variables).await
    }

    pub async fn originate_with_id<'a>(
        &self,
        channel_id: &str,
        params: OriginateChannelWithIdParams<'a>,
        variables: &HashMap<&str, &str>,
    ) -> Result<Channel> {
        self.authorized_post_variables(["channels", channel_id], params, variables).await
    }

    pub fn start_moh(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn stop_moh(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn start_silence(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn stop_silence(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }
    pub fn get_variable(&self, _channel_id: &str) -> Result<ChannelVariable> {
        unimplemented!()
    }

    pub fn set_variable(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn continue_in_dialplan(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    /// Transfer the channel to another ARI application.
    /// Same as `move` in Asterisk
    pub fn transfer(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn get_rtp_statistics(&self, _channel_id: &str) -> Result<RtpStatistics> {
        unimplemented!()
    }

    pub fn snoop(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn snoop_with_id(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn start_external_media(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }
}
