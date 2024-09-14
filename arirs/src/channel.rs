use std::collections::HashMap;

use chrono::DateTime;
use derive_getters::Getters;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use url::Url;

use crate::*;

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Channel {
    id: String,
    name: String,
    state: String,
    protocol_id: String,
    caller: Caller,
    connected: Caller,
    accountcode: String,
    dialplan: Dialplan,
    creationtime: String,
    language: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginateChannelParams<'a> {
    pub endpoint: &'a str,
    pub params: OriginateParams<'a>,
    pub caller_id: Option<&'a str>,
    pub timeout: Option<u32>,
    pub channel_id: Option<&'a str>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginateChannelWithIdParams<'a> {
    pub endpoint: &'a str,
    pub params: OriginateParams<'a>,
    pub caller_id: Option<&'a str>,
    pub timeout: Option<u32>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OriginateParams<'a> {
    Extension {
        extension: &'a str,
        context: Option<&'a str>,
        priority: Option<i32>,
        label: Option<&'a str>,
    },
    Application {
        app: &'a str,
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        #[serde(serialize_with = "join_serialize")]
        app_args: &'a [&'a str],
    },
}

pub use reason::Reason;
mod reason {
    use serde::ser::SerializeMap;
    use strum::AsRefStr;

    use super::*;

    #[derive(Debug, AsRefStr)]
    pub enum Reason {
        Code(u16),
        #[strum(serialize = "normal")]
        Normal,
        #[strum(serialize = "busy")]
        Busy,
        #[strum(serialize = "congestion")]
        Congestion,
        #[strum(serialize = "no_answer")]
        NoAnswer,
        #[strum(serialize = "timeout")]
        Timeout,
        #[strum(serialize = "rejected")]
        Rejected,
        #[strum(serialize = "unallocated")]
        Unallocated,
        #[strum(serialize = "normal_unspecified")]
        NormalUnspecified,
        #[strum(serialize = "number_incomplete")]
        NumberIncomplete,
        #[strum(serialize = "codec_mismatch")]
        CodecMismatch,
        #[strum(serialize = "interworking")]
        Interworking,
        #[strum(serialize = "failure")]
        Failure,
        #[strum(serialize = "answered_elsewhere")]
        AnsweredElsewhere,
    }

    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut map = serializer.serialize_map(Some(1))?;
            match self {
                Reason::Code(code) => map.serialize_entry("reason_code", code)?,
                _ => map.serialize_entry("reason", self.as_ref())?,
            };
            map.end()
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "direction")]
pub enum Direction {
    In,
    Out,
    Both,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    timestamp: DateTime<chrono::Utc>,
    args: Vec<String>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    timestamp: DateTime<chrono::Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    timestamp: DateTime<chrono::Utc>,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    timestamp: DateTime<chrono::Utc>,
    cause: i32,
    cause_txt: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    timestamp: DateTime<chrono::Utc>,
    variable: String,
    value: String,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    timestamp: DateTime<chrono::Utc>,
    soft: Option<bool>,
    cause: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    timestamp: DateTime<chrono::Utc>,
    dialplan_app: String,
    dialplan_app_data: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelStateChange {
    timestamp: DateTime<chrono::Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    timestamp: DateTime<chrono::Utc>,
    digit: String,
    duration_ms: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Caller {
    name: String,
    number: String,
}

#[derive(Serialize, Deserialize, Debug, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Dialplan {
    context: String,
    exten: String,
    priority: i32,
    app_name: String,
    app_data: String,
}

#[derive(Serialize)]
pub struct SendDtmfParams<'a> {
    pub dtmf: &'a str,
    /// in milliseconds
    pub between: Option<u32>,
    /// in milliseconds
    pub duration: Option<u32>,
    pub before: Option<u32>,
    pub after: Option<u32>,
}

#[derive(Serialize)]
pub struct PlayMediaParams<'a> {
    pub media: &'a str,
    pub lang: Option<&'a str>,
    pub offset_ms: Option<u32>,
    pub skip_ms: Option<u32>,
    pub playback_id: Option<&'a str>,
}

#[derive(Serialize)]
pub struct PlayMediaWithIdParams<'a> {
    #[serde(serialize_with = "join_serialize")]
    pub media: &'a [&'a str],
    pub lang: Option<&'a str>,
    pub offset_ms: Option<u32>,
    pub skip_ms: Option<u32>,
}

fn join_serialize<S>(slice: &[&str], s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&slice.join(","))
}

#[derive(Serialize)]
pub struct RecordParams<'a> {
    pub name: &'a str,
    pub format: &'a str,
    pub max_duration_seconds: Option<u32>,
    pub max_silence_seconds: Option<u32>,
    pub if_exists: RecordingAction,
    pub beep: bool,
    pub terminate_on: RecordingTermination,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordingAction {
    Overwrite,
    Append,
    Fail,
}
#[derive(Debug, Serialize)]
pub enum RecordingTermination {
    None,
    Any,
    #[serde(rename = "*")]
    Asterisk,
    #[serde(rename = "#")]
    Octothorpe,
}

#[derive(Serialize)]
pub struct DialParams<'a> {
    pub caller: Option<&'a str>,
    pub timeout: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelCreateParams<'a> {
    pub endpoint: &'a str,
    pub app: &'a str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub app_args: &'a [&'a str],
    pub channel_id: Option<&'a str>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

impl RequestClient {
    pub async fn hangup(&self, channel_id: &str, reason: Reason) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}", channel_id))?;
        self.set_authorized_query_params(&mut url, reason);

        self.as_ref().delete(url).send().await?;

        Ok(())
    }

    pub async fn answer(&self, channel_id: &str) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/answer", channel_id))?;
        self.set_authorized_query_params(&mut url, ());

        self.as_ref().post(url).send().await?;
        Ok(())
    }

    pub async fn start_ringing(&self, channel_id: &str) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/ring", channel_id))?;
        self.set_authorized_query_params(&mut url, ());

        self.as_ref().post(url).send().await?;
        Ok(())
    }

    pub async fn stop_ringing(&self, channel_id: &str) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/ring", channel_id))?;
        self.set_authorized_query_params(&mut url, ());

        self.as_ref().delete(url).send().await?;
        Ok(())
    }

    pub async fn send_dtmf(&self, channel_id: &str, params: SendDtmfParams<'_>) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/dtmf", channel_id))?;
        self.set_authorized_query_params(&mut url, params);

        self.as_ref().post(url).send().await?;

        Ok(())
    }

    pub async fn mute(&self, channel_id: &str, direction: Direction) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/mute", channel_id))?;
        self.set_authorized_query_params(&mut url, direction);

        self.as_ref().post(url).send().await?;
        Ok(())
    }

    pub async fn unmute(&self, channel_id: &str, direction: Direction) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/mute", channel_id))?;
        self.set_authorized_query_params(&mut url, direction);

        self.as_ref().delete(url).send().await?;
        Ok(())
    }

    pub async fn hold(&self, channel_id: &str) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/hold", channel_id))?;
        self.set_authorized_query_params(&mut url, ());

        self.as_ref().post(url).send().await?;
        Ok(())
    }

    pub async fn unhold(&self, channel_id: &str) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/ring", channel_id))?;
        self.set_authorized_query_params(&mut url, ());

        self.as_ref().delete(url).send().await?;
        Ok(())
    }

    pub async fn play_media(&self, channel_id: &str, params: PlayMediaParams<'_>) -> Result<Playback> {
        let mut url = self.url().join(&format!("channels/{}/play", channel_id))?;
        self.set_authorized_query_params(&mut url, params);

        let playback = self.as_ref().post(url).send().await?.json::<Playback>().await?;
        Ok(playback)
    }

    pub async fn play_media_with_id(&self, channel_id: &str, playback_id: &str, params: PlayMediaWithIdParams<'_>) -> Result<Playback> {
        let mut url = self.url().join(&format!("channels/{}/play/{}/media", channel_id, playback_id))?;
        self.set_authorized_query_params(&mut url, params);

        let playback = self.as_ref().post(url).send().await?.json().await?;
        Ok(playback)
    }

    pub async fn record(&self, channel_id: &str, params: RecordParams<'_>) -> Result<LiveRecording> {
        let mut url = self.url().join(&format!("channels/{}/record", channel_id))?;
        self.set_authorized_query_params(&mut url, params);

        let recording = self.as_ref().post(url).send().await?.json().await?;
        Ok(recording)
    }

    pub async fn dial(&self, channel_id: &str, params: DialParams<'_>) -> Result<()> {
        let mut url = self.url().join(&format!("channels/{}/dial", channel_id))?;
        self.set_authorized_query_params(&mut url, params);

        self.as_ref().post(url).send().await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Channel>> {
        let url: Url = self
            .url()
            .join("channels")?
            .query_pairs_mut()
            .append_pair("api_key", &self.get_api_key())
            .finish()
            .to_owned();

        let channels = reqwest::get(url).await?.json::<Vec<Channel>>().await?;
        Ok(channels)
    }

    pub async fn create(&self, params: ChannelCreateParams<'_>, variables: &HashMap<&str, &str>) -> Result<Channel> {
        let mut url = self.url().join("channels")?;
        self.set_authorized_query_params(&mut url, params);

        let channel = self
            .as_ref()
            .post(url)
            .json(&json!({
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(channel)
    }

    pub async fn get(self, channel_id: &str) -> Result<Channel> {
        let url = self
            .url()
            .join(&format!("channels/{}", channel_id))?
            .query_pairs_mut()
            .append_pair("api_key", &self.get_api_key())
            .finish()
            .to_owned();

        let channel = reqwest::get(url).await?.json::<Channel>().await?;
        Ok(channel)
    }

    pub async fn originate<'a>(&self, params: OriginateChannelParams<'a>, variables: &HashMap<&str, &str>) -> Result<Channel> {
        let mut url = self.url().join("channels")?;

        self.set_authorized_query_params(&mut url, params);

        let channel = self
            .as_ref()
            .post(url)
            .json(&json!({
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(channel)
    }

    pub async fn originate_with_id<'a>(
        &self,
        channel_id: &str,
        params: OriginateChannelWithIdParams<'a>,
        variables: &HashMap<&str, &str>,
    ) -> Result<Channel> {
        let mut url = self.url().join(&format!("channels/{}", channel_id))?;
        self.set_authorized_query_params(&mut url, params);

        let channel = self
            .as_ref()
            .post(url)
            .json(&json!({
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(channel)
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
    pub fn get_variable(&self, _channel_id: &str) -> Result<Variable> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_parameters() {
        let request_client = RequestClient::new("http://localhost:8080/".parse().unwrap(), "asterisk", "asterisk");

        let mut url = request_client.url().join("channel").unwrap();

        request_client.set_authorized_query_params(
            &mut url,
            PlayMediaParams {
                media: "sound:hello",
                lang: Some("en"),
                offset_ms: None,
                skip_ms: None,
                playback_id: None,
            },
        );

        let expected = "http://localhost:8080/channel?api_key=asterisk%3Aasterisk&media=sound%3Ahello&lang=en";
        assert_eq!(expected, url.as_str())
    }

    #[test]
    fn serializes_unit_type() {
        let request_client = RequestClient::new("http://localhost:8080/".parse().unwrap(), "asterisk", "asterisk");

        let mut url = request_client.url().join("channel").unwrap();

        request_client.set_authorized_query_params(&mut url, ());

        let expected = "http://localhost:8080/channel?api_key=asterisk%3Aasterisk";
        assert_eq!(expected, url.as_str())
    }
}
