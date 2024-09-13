use std::collections::HashMap;

use chrono::{DateTime, Duration};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{event, Level};
use url::Url;

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub state: String,
    pub protocol_id: String,
    pub caller: Caller,
    pub connected: Caller,
    pub accountcode: String,
    pub dialplan: Dialplan,
    pub creationtime: String,
    pub language: String,
}

#[derive(Debug)]
pub enum OriginateParams<'a> {
    Extension {
        extension: &'a str,
        context: Option<&'a str>,
        priority: Option<i32>,
        label: Option<&'a str>,
    },
    Application {
        app: &'a str,
        app_args: Vec<&'a str>,
    },
}

#[derive(Debug, Display)]
pub enum Reason {
    #[display("{}", _0)]
    Code(u16),
    #[display("normal")]
    Normal,
    #[display("busy")]
    Busy,
    #[display("congestion")]
    Congestion,
    #[display("no_answer")]
    NoAnswer,
    #[display("timeout")]
    Timeout,
    #[display("rejected")]
    Rejected,
    #[display("unallocated")]
    Unallocated,
    #[display("normal_unspecified")]
    NormalUnspecified,
    #[display("number_incomplete")]
    NumberIncomplete,
    #[display("codec_mismatch")]
    CodecMismatch,
    #[display("interworking")]
    Interworking,
    #[display("failure")]
    Failure,
    #[display("answered_elsewhere")]
    AnsweredElsewhere,
}

#[derive(Debug, Display)]
pub enum Direction {
    #[display("in")]
    In,
    #[display("out")]
    Out,
    #[display("both")]
    Both,
}

#[derive(Debug, Display)]
pub enum RecordingAction {
    #[display("overwrite")]
    Overwrite,
    #[display("append")]
    Append,
    #[display("fail")]
    Fail,
}

#[derive(Debug, Display)]
pub enum RecordingTermination {
    #[display("none")]
    None,
    #[display("any")]
    Any,
    #[display("*")]
    Asterisk,
    #[display("#")]
    Octothorpe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    pub timestamp: DateTime<chrono::Utc>,
    pub args: Vec<String>,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    pub timestamp: DateTime<chrono::Utc>,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    pub timestamp: DateTime<chrono::Utc>,
    pub channel: Option<Channel>,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    pub timestamp: DateTime<chrono::Utc>,
    pub cause: i32,
    pub cause_txt: String,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    pub timestamp: DateTime<chrono::Utc>,
    pub variable: String,
    pub value: String,
    pub channel: Option<Channel>,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    pub timestamp: DateTime<chrono::Utc>,
    pub soft: Option<bool>,
    pub cause: i32,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    pub timestamp: DateTime<chrono::Utc>,
    pub dialplan_app: String,
    pub dialplan_app_data: String,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelStateChange {
    pub timestamp: DateTime<chrono::Utc>,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    pub timestamp: DateTime<chrono::Utc>,
    pub digit: String,
    pub duration_ms: i32,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Caller {
    pub name: String,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Dialplan {
    pub context: String,
    pub exten: String,
    pub priority: i32,
    pub app_name: String,
    pub app_data: String,
}

impl Channel {
    pub async fn hangup(self, client: &Client, reason: Reason) -> Result<()> {
        let mut url = client.url.join(&format!("channels/{}", self.id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        match reason {
            Reason::Code(_) => url.append_pair("reason_code", &format!("{}", reason)),
            _ => url.append_pair("reason", &format!("{}", reason)),
        };

        reqwest::Client::new().delete(url.finish().to_owned()).send().await?;

        event!(Level::INFO, "hung up channel with id {}", self.id);
        Ok(())
    }

    pub fn continue_in_dialplan(self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    /// Transfer the channel to another ARI application.
    /// Same as `move` in Asterisk
    pub fn transfer(self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn answer(&self, client: &Client) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/answer", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        reqwest::Client::new().post(url).send().await?;
        event!(Level::INFO, "answered channel with id {}", self.id);
        Ok(())
    }

    pub async fn start_ringing(&self, client: &Client) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/ring", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        reqwest::Client::new().post(url).send().await?;
        event!(Level::INFO, "started ringing channel with id {}", self.id);
        Ok(())
    }

    pub async fn stop_ringing(&self, client: &Client) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/ring", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &format!("{}:{}", client.username, client.password))
            .finish()
            .to_owned();

        reqwest::Client::new().delete(url).send().await?;
        event!(Level::INFO, "stopped ringing channel with id {}", self.id);
        Ok(())
    }

    pub async fn send_dtmf(
        &self,
        client: &Client,
        dtmf: &str,
        before: Option<Duration>,
        between: Option<Duration>,
        duration: Option<Duration>,
        after: Option<Duration>,
    ) -> Result<()> {
        let mut url = client.url.join(&format!("channels/{}/dtmf", self.id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        url.append_pair("dtmf", dtmf)
            .append_pair("between", &between.map(|d| d.num_milliseconds()).unwrap_or(100).to_string())
            .append_pair("duration", &duration.map(|d| d.num_milliseconds()).unwrap_or(100).to_string());

        if let Some(before) = before {
            url.append_pair("before", &before.num_milliseconds().to_string());
        }

        if let Some(after) = after {
            url.append_pair("after", &after.num_milliseconds().to_string());
        }

        reqwest::Client::new().post(url.finish().to_owned()).send().await?;

        event!(Level::INFO, "sent dtmf '{}' to channel with id {}", dtmf, self.id);

        Ok(())
    }

    pub async fn mute(&self, client: &Client, direction: Direction) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/mute", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .append_pair("direction", &format!("{}", direction))
            .finish()
            .to_owned();

        reqwest::Client::new().post(url).send().await?;
        event!(Level::INFO, "muted channel with id {}", self.id);
        Ok(())
    }

    pub async fn unmute(&self, client: &Client, direction: Direction) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/mute", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .append_pair("direction", &format!("{}", direction))
            .finish()
            .to_owned();

        reqwest::Client::new().delete(url).send().await?;
        event!(Level::INFO, "unmuted channel with id {}", self.id);
        Ok(())
    }

    pub async fn hold(&self, client: &Client) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/hold", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        reqwest::Client::new().post(url).send().await?;
        event!(Level::INFO, "started hold on channel with id {}", self.id);
        Ok(())
    }

    pub async fn unhold(&self, client: &Client) -> Result<()> {
        let url = client
            .url
            .join(&format!("channels/{}/hold", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        reqwest::Client::new().delete(url).send().await?;
        event!(Level::INFO, "stopped hold on channel with id {}", self.id);
        Ok(())
    }

    pub fn start_moh(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub fn stop_moh(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub fn start_silence(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub fn stop_silence(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn play_media(
        &self,
        client: &Client,
        media: &str,
        lang: Option<&str>,
        offset_ms: Option<u32>,
        skip_ms: Option<u32>,
        playback_id: Option<&str>,
    ) -> Result<Playback> {
        let mut url = client.url.join(&format!("channels/{}/play", self.id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);
        url.append_pair("media", media);

        if let Some(lang) = lang {
            url.append_pair("lang", lang);
        }

        if let Some(offset_ms) = offset_ms {
            url.append_pair("offset_ms", &offset_ms.to_string());
        }

        if let Some(skip_ms) = skip_ms {
            url.append_pair("skip_ms", &skip_ms.to_string());
        }

        if let Some(playback_id) = playback_id {
            url.append_pair("playback_id", playback_id);
        }

        let playback = reqwest::Client::new()
            .post(url.finish().to_owned())
            .send()
            .await?
            .json::<Playback>()
            .await?;

        event!(
            Level::INFO,
            "started media playback with id {} on channel with id {}",
            playback.id,
            self.id
        );

        Ok(playback)
    }

    pub async fn play_media_with_id(
        &self,
        client: &Client,
        playback_id: &str,
        media: Vec<&str>,
        lang: Option<&str>,
        offset_ms: Option<u32>,
        skip_ms: Option<u32>,
    ) -> Result<Playback> {
        let mut url = client.url.join(&format!("channels/{}/play/{}/media", self.id, playback_id))?;

        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);
        let media = media.join(",");
        url.append_pair("media", &media);

        if let Some(lang) = lang {
            url.append_pair("lang", lang);
        }

        if let Some(offset_ms) = offset_ms {
            url.append_pair("offset_ms", &offset_ms.to_string());
        }

        if let Some(skip_ms) = skip_ms {
            url.append_pair("skip_ms", &skip_ms.to_string());
        }

        let playback = reqwest::Client::new()
            .post(url.finish().to_owned())
            .send()
            .await?
            .json::<Playback>()
            .await?;

        event!(
            Level::INFO,
            "started media playback with id {} on channel with id {}",
            playback.id,
            self.id
        );

        Ok(playback)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn record(
        &self,
        client: &Client,
        name: &str,
        format: &str,
        max_duration_seconds: Option<u32>,
        max_silence_seconds: Option<u32>,
        if_exists: RecordingAction,
        beep: bool,
        terminate_on: RecordingTermination,
    ) -> Result<LiveRecording> {
        let mut url = client.url.join(&format!("channels/{}/record", self.id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        url.append_pair("name", name)
            .append_pair("format", format)
            .append_pair("if_exists", &format!("{}", if_exists))
            .append_pair("beep", &beep.to_string())
            .append_pair("terminate_on", &format!("{}", terminate_on));

        if let Some(max_duration_seconds) = max_duration_seconds {
            url.append_pair("max_duration_seconds", &max_duration_seconds.to_string());
        }

        if let Some(max_silence_seconds) = max_silence_seconds {
            url.append_pair("max_silence_seconds", &max_silence_seconds.to_string());
        }

        let recording = reqwest::Client::new()
            .post(url.finish().to_owned())
            .send()
            .await?
            .json::<LiveRecording>()
            .await?;

        event!(
            Level::INFO,
            "started recording with id {} on channel with id {}",
            recording.id,
            self.id
        );

        Ok(recording)
    }

    pub fn get_variable(&self, _client: &Client) -> Result<Variable> {
        unimplemented!()
    }

    pub fn set_variable(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn dial(&self, client: &Client, caller_id: Option<&str>, timeout: Option<u32>) -> Result<()> {
        let mut url = client.url.join(&format!("channels/{}/dial", self.id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", caller_id);
        }

        if let Some(timeout) = timeout {
            url.append_pair("timeout", &timeout.to_string());
        }

        reqwest::Client::new().post(url.finish().to_owned()).send().await?;

        event!(Level::INFO, "dialed channel with id {}", self.id);
        Ok(())
    }

    pub fn get_rtp_statistics(&self, _client: &Client) -> Result<RtpStatistics> {
        unimplemented!()
    }

    pub async fn list(client: &Client) -> Result<Vec<Channel>> {
        let url: Url = client
            .url
            .join("channels")?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let channels = reqwest::get(url).await?.json::<Vec<Channel>>().await?;
        event!(Level::INFO, "received channels");
        Ok(channels)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        client: &Client,
        endpoint: &str,
        app: &str,
        app_args: Vec<&str>,
        channel_id: Option<&str>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: HashMap<&str, &str>,
    ) -> Result<Channel> {
        let mut url = client.url.join("channels")?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);
        url.append_pair("endpoint", endpoint).append_pair("app", app);

        if !formats.is_empty() {
            let formats = formats.join(",");
            url.append_pair("formats", &formats);
        }

        if !app_args.is_empty() {
            let app_args = app_args.join(",");
            url.append_pair("app_args", &app_args);
        }

        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", channel_id);
        }

        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", other_channel_id);
        }

        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        let body = json!({
            "variables": variables
        });

        let channel = reqwest::Client::new()
            .post(url.finish().to_owned())
            .json(&body)
            .send()
            .await?
            .json::<Channel>()
            .await?;

        event!(Level::INFO, "created channel with id {}", channel.id);
        Ok(channel)
    }

    pub async fn get(client: &Client, channel_id: &str) -> Result<Channel> {
        let url = client
            .url
            .join(&format!("channels/{}", channel_id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let channel = reqwest::get(url).await?.json::<Channel>().await?;
        event!(Level::INFO, "received channel with id {}", channel.id);
        Ok(channel)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn originate<'a>(
        client: &Client,
        endpoint: &str,
        params: OriginateParams<'a>,
        caller_id: Option<&str>,
        timeout: Option<u32>,
        channel_id: Option<&str>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: HashMap<&str, &str>,
    ) -> Result<Channel> {
        let mut url = client.url.join("channels")?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        url.append_pair("endpoint", endpoint)
            .append_pair("timeout", &timeout.unwrap_or(30).to_string());

        if !formats.is_empty() {
            let formats = formats.join(",");
            url.append_pair("formats", &formats);
        }

        match params {
            OriginateParams::Extension {
                extension,
                context,
                priority,
                label,
            } => {
                url.append_pair("extension", extension);

                if let Some(context) = context {
                    url.append_pair("context", context);
                }

                if let Some(priority) = priority {
                    url.append_pair("priority", &priority.to_string());
                }

                if let Some(label) = label {
                    url.append_pair("label", label);
                }
            }
            OriginateParams::Application { app, app_args } => {
                url.append_pair("app", app);

                if !app_args.is_empty() {
                    let app_args = app_args.join(",");
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", caller_id);
        }

        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", channel_id);
        }

        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", other_channel_id);
        }

        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        let body = json!({
            "variables": variables
        });

        let channel = reqwest::Client::new()
            .post(url.finish().to_owned())
            .json(&body)
            .send()
            .await?
            .json::<Channel>()
            .await?;

        event!(Level::INFO, "originated channel with id {}", channel.id);
        Ok(channel)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn originate_with_id<'a>(
        client: &Client,
        channel_id: &str,
        endpoint: &str,
        params: OriginateParams<'a>,
        caller_id: Option<&str>,
        timeout: Option<u32>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: HashMap<&str, &str>,
    ) -> Result<Channel> {
        let mut url = client.url.join(&format!("channels/{}", channel_id))?;
        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        url.append_pair("endpoint", endpoint)
            .append_pair("timeout", &timeout.unwrap_or(30).to_string());

        if !formats.is_empty() {
            let formats = formats.join(",");
            url.append_pair("formats", &formats);
        }

        match params {
            OriginateParams::Extension {
                extension,
                context,
                priority,
                label,
            } => {
                url.append_pair("extension", extension);

                if let Some(context) = context {
                    url.append_pair("context", context);
                }

                if let Some(priority) = priority {
                    url.append_pair("priority", &priority.to_string());
                }

                if let Some(label) = label {
                    url.append_pair("label", label);
                }
            }
            OriginateParams::Application { app, app_args } => {
                url.append_pair("app", app);

                if !app_args.is_empty() {
                    let app_args = app_args.join(",");
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", caller_id);
        }

        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("otherChannelId", other_channel_id);
        }

        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        let body = json!({
            "variables": variables
        });

        let channel = reqwest::Client::new()
            .post(url.finish().to_owned())
            .json(&body)
            .send()
            .await?
            .json::<Channel>()
            .await?;

        event!(Level::INFO, "originated channel with id {}", channel.id);
        Ok(channel)
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
