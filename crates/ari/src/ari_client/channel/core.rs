use std::collections::HashMap;

use crate::*;

impl AriClient {
    pub async fn channel_answer(&self, channel_id: &str) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "answer"], ()).await
    }

    pub async fn channel_hangup(&self, channel_id: &str, reason: Reason) -> AriClientResult<()> {
        self.authorized_delete(["channels", channel_id], reason).await
    }

    pub async fn channel_start_ringing(&self, channel_id: &str) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "ring"], ()).await
    }

    pub async fn channel_stop_ringing(&self, channel_id: &str) -> AriClientResult<()> {
        self.authorized_delete(["channels", channel_id, "ring"], ()).await
    }

    pub async fn channel_send_dtmf(&self, channel_id: &str, params: SendDtmfParams<'_>) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "dtmf"], params).await
    }

    pub async fn channel_mute(&self, channel_id: &str, direction: Direction) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "mute"], direction).await
    }

    pub async fn channel_unmute(&self, channel_id: &str, direction: Direction) -> AriClientResult<()> {
        self.authorized_delete(["channels", channel_id, "mute"], direction).await
    }

    pub async fn channel_hold(&self, channel_id: &str) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "hold"], ()).await
    }

    pub async fn channel_unhold(&self, channel_id: &str) -> AriClientResult<()> {
        self.authorized_delete(["channels", channel_id, "hold"], ()).await
    }

    pub async fn channel_play_media(&self, channel_id: &str, params: PlayMediaParams<'_>) -> AriClientResult<Playback> {
        self.authorized_post_json_response(["channels", channel_id, "play"], params).await
    }

    // SUGGESTION(gibbz00): combine with above method and mave ID optional
    pub async fn channel_play_media_with_id(
        &self,
        channel_id: &str,
        playback_id: &str,
        params: PlayMediaBaseParams<'_>,
    ) -> AriClientResult<Playback> {
        self.authorized_post_json_response(["channels", channel_id, "play", playback_id, "media"], params)
            .await
    }

    pub async fn channel_record(&self, channel_id: &str, params: RecordParams<'_>) -> AriClientResult<LiveRecording> {
        self.authorized_post_json_response(["channels", channel_id, "record"], params).await
    }

    pub async fn channel_dial(&self, channel_id: &str, params: DialParams<'_>) -> AriClientResult<()> {
        self.authorized_post(["channels", channel_id, "dial"], params).await
    }

    pub async fn channel_list(&self) -> AriClientResult<Vec<Channel>> {
        self.authorized_get(["channels"], ()).await
    }

    pub async fn channel_create(&self, params: ChannelCreateParams<'_>, variables: &HashMap<&str, &str>) -> AriClientResult<Channel> {
        self.authorized_post_variables(["channels", "create"], params, variables).await
    }

    pub async fn channel_get(self, channel_id: &str) -> AriClientResult<Channel> {
        self.authorized_get(["channels", channel_id], ()).await
    }

    pub async fn channel_originate<'a>(
        &self,
        params: OriginateChannelParams<'a>,
        variables: &HashMap<&str, &str>,
    ) -> AriClientResult<Channel> {
        self.authorized_post_variables(["channels"], params, variables).await
    }

    // SUGGESTION(gibbz00): combine with above method and mave ID optional
    pub async fn channel_originate_with_id<'a>(
        &self,
        channel_id: &str,
        params: OriginateChannelWithIdParams<'a>,
        variables: &HashMap<&str, &str>,
    ) -> AriClientResult<Channel> {
        self.authorized_post_variables(["channels", channel_id], params, variables).await
    }

    pub async fn channel_start_moh(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn channel_stop_moh(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn channel_silence(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn channel_unsilince(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }
    pub async fn channel_get_variable(&self, _channel_id: &str) -> AriClientResult<ChannelVariable> {
        unimplemented!()
    }

    pub async fn channel_set_variable(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn channel_continue_in_dialplan(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    /// Transfer the channel to another ARI application.
    /// Same as `move` in Asterisk
    pub async fn channel_transfer(&self, _channel_id: &str) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn channel_get_rtp_statistics(&self, _channel_id: &str) -> AriClientResult<RtpStatistics> {
        unimplemented!()
    }

    pub async fn channel_snoop(&self, _channel_id: &str) -> AriClientResult<Channel> {
        unimplemented!()
    }

    // SUGGESTION(gibbz00): combine with above method and mave ID optional
    pub async fn channel_snoop_with_id(&self, _channel_id: &str) -> AriClientResult<Channel> {
        unimplemented!()
    }

    pub async fn channel_start_external_media(&self, _channel_id: &str) -> AriClientResult<Channel> {
        unimplemented!()
    }
}
