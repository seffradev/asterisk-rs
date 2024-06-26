use crate::{
    client::Client, playback::Playback, recording::LiveRecording, rtp_stat::RtpStat,
    variable::Variable,
};
use crate::{AriError, Result};
use chrono::DateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tracing::{event, span, Level};
use url::Url;

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
    #[display(fmt = "{}", _0)]
    Code(u16),
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "busy")]
    Busy,
    #[display(fmt = "congestion")]
    Congestion,
    #[display(fmt = "no_answer")]
    NoAnswer,
    #[display(fmt = "timeout")]
    Timeout,
    #[display(fmt = "rejected")]
    Rejected,
    #[display(fmt = "unallocated")]
    Unallocated,
    #[display(fmt = "normal_unspecified")]
    NormalUnspecified,
    #[display(fmt = "number_incomplete")]
    NumberIncomplete,
    #[display(fmt = "codec_mismatch")]
    CodecMismatch,
    #[display(fmt = "interworking")]
    Interworking,
    #[display(fmt = "failure")]
    Failure,
    #[display(fmt = "answered_elsewhere")]
    AnsweredElsewhere,
}

#[derive(Debug, Display)]
pub enum Direction {
    #[display(fmt = "in")]
    In,
    #[display(fmt = "out")]
    Out,
    #[display(fmt = "both")]
    Both,
}

#[derive(Debug, Display)]
pub enum RecordingAction {
    #[display(fmt = "overwrite")]
    Overwrite,
    #[display(fmt = "append")]
    Append,
    #[display(fmt = "fail")]
    Fail,
}

#[derive(Debug, Display)]
pub enum RecordingTermination {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "any")]
    Any,
    #[display(fmt = "*")]
    Asterisk,
    #[display(fmt = "#")]
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
        let span = span!(Level::INFO, "hangup");
        let _guard = span.enter();

        let mut url = client.url.join(&format!("/ari/channels/{}", self.id))?;

        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);

        match reason {
            Reason::Code(_) => url.append_pair("reason_code", &format!("{}", reason)),
            _ => url.append_pair("reason", &format!("{}", reason)),
        };

        let url = url.finish().to_owned();

        let response = reqwest::Client::new().delete(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to hang up channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not hang up channel"),
            ));
        }

        event!(Level::INFO, "Successfully hung up channel");
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
        let span = span!(Level::INFO, "answer");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}/answer", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to answer channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not answer channel"),
            ));
        }

        event!(Level::INFO, "Successfully answered channel");
        Ok(())
    }

    pub async fn start_ringing(&self, client: &Client) -> Result<()> {
        let span = span!(Level::INFO, "start_ringing");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}/ring", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to ring channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not ring channel"),
            ));
        }

        event!(Level::INFO, "Successfully rang channel");
        Ok(())
    }

    pub async fn stop_ringing(&self, client: &Client) -> Result<()> {
        let span = span!(Level::INFO, "stop_ringing");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}/ring", self.id))?
            .query_pairs_mut()
            .append_pair(
                "api_key",
                &format!("{}:{}", client.username, client.password),
            )
            .finish()
            .to_owned();

        let response = reqwest::Client::new().delete(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to stop ringing channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not stop ringing channel"),
            ));
        }

        event!(Level::INFO, "Successfully stopped ringing channel");
        Ok(())
    }

    pub fn send_dtmf(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn mute(&self, client: &Client, direction: Direction) -> Result<()> {
        let span = span!(Level::INFO, "mute");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}/mute", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .append_pair("direction", &format!("{}", direction))
            .finish()
            .to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to mute channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not mute channel"),
            ));
        }

        event!(Level::INFO, "Successfully muted channel");
        Ok(())
    }

    pub async fn unmute(&self, client: &Client, direction: Direction) -> Result<()> {
        let span = span!(Level::INFO, "unmute");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}/mute", self.id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .append_pair("direction", &format!("{}", direction))
            .finish()
            .to_owned();

        let response = reqwest::Client::new().delete(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to unmute channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not unmute channel"),
            ));
        }

        event!(Level::INFO, "Successfully unmuted channel");
        Ok(())
    }

    pub fn hold(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub fn unhold(&self, _client: &Client) -> Result<()> {
        unimplemented!()
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
        let span = span!(Level::INFO, "play_media");
        let _guard = span.enter();

        let mut url = client
            .url
            .join(&format!("/ari/channels/{}/play", self.id))?;

        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);
        url.append_pair("media", media);

        if let Some(lang) = lang {
            event!(Level::INFO, "Lang: {}", lang);
            url.append_pair("lang", lang);
        }

        if let Some(offset_ms) = offset_ms {
            event!(Level::INFO, "Offset: {}", offset_ms);
            url.append_pair("offset_ms", &offset_ms.to_string());
        }

        if let Some(skip_ms) = skip_ms {
            event!(Level::INFO, "Skip: {}", skip_ms);
            url.append_pair("skip_ms", &skip_ms.to_string());
        }

        if let Some(playback_id) = playback_id {
            event!(Level::INFO, "Playback ID: {}", playback_id);
            url.append_pair("playback_id", playback_id);
        }

        let url = url.finish().to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to play media");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not play media"),
            ));
        }

        let playback = response.json::<Playback>().await?;

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
        let span = span!(Level::INFO, "play_media_with_id");
        let _guard = span.enter();

        let mut url = client.url.join(&format!(
            "/ari/channels/{}/play/{}/media",
            self.id, playback_id
        ))?;

        let mut url = url.query_pairs_mut();
        client.add_api_key(&mut url);

        let media = media.join(",");
        event!(Level::INFO, "Media: {}", media);
        url.append_pair("media", &media);

        if let Some(lang) = lang {
            event!(Level::INFO, "Lang: {}", lang);
            url.append_pair("lang", lang);
        }

        if let Some(offset_ms) = offset_ms {
            event!(Level::INFO, "Offset: {}", offset_ms);
            url.append_pair("offset_ms", &offset_ms.to_string());
        }

        if let Some(skip_ms) = skip_ms {
            event!(Level::INFO, "Skip: {}", skip_ms);
            url.append_pair("skip_ms", &skip_ms.to_string());
        }

        let url = url.finish().to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to play media");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not play media"),
            ));
        }

        let playback = response.json::<Playback>().await?;
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
        let span = span!(Level::INFO, "record");
        let _guard = span.enter();

        let mut url = client
            .url
            .join(&format!("/ari/channels/{}/record", self.id))?;

        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);

        url.append_pair("name", name).append_pair("format", format);

        if let Some(max_duration_seconds) = max_duration_seconds {
            event!(Level::INFO, "Max duration: {}", max_duration_seconds);
            url.append_pair("max_duration_seconds", &max_duration_seconds.to_string());
        }

        if let Some(max_silence_seconds) = max_silence_seconds {
            event!(Level::INFO, "Max silence: {}", max_silence_seconds);
            url.append_pair("max_silence_seconds", &max_silence_seconds.to_string());
        }

        event!(Level::INFO, "If exists: {}", if_exists);
        url.append_pair("if_exists", &format!("{}", if_exists));

        event!(Level::INFO, "Beep: {}", beep);
        url.append_pair("beep", &beep.to_string());

        event!(Level::INFO, "Terminate on: {}", terminate_on);
        url.append_pair("terminate_on", &format!("{}", terminate_on));

        let url = url.finish().to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to record channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not record channel"),
            ));
        }

        let recording = response.json::<LiveRecording>().await?;
        Ok(recording)
    }

    pub fn get_variable(&self, _client: &Client) -> Result<Variable> {
        unimplemented!()
    }

    pub fn set_variable(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    pub async fn dial(
        &self,
        client: &Client,
        caller_id: Option<&str>,
        timeout: Option<u32>,
    ) -> Result<()> {
        let span = span!(Level::INFO, "dial");
        let _guard = span.enter();

        let mut url = client
            .url
            .join(&format!("/ari/channels/{}/dial", self.id))?;

        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);

        if let Some(caller_id) = caller_id {
            event!(Level::INFO, "Caller ID: {}", caller_id);
            url.append_pair("callerId", caller_id);
        }

        if let Some(timeout) = timeout {
            event!(Level::INFO, "Timeout: {}", timeout);
            url.append_pair("timeout", &timeout.to_string());
        }

        let url = url.finish().to_owned();

        let response = reqwest::Client::new().post(url).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to dial channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not dial channel"),
            ));
        }

        event!(Level::INFO, "Successfully dialed channel");
        Ok(())
    }

    pub fn get_rtp_stat(&self, _client: &Client) -> Result<RtpStat> {
        unimplemented!()
    }

    pub async fn list(client: &Client) -> Result<Vec<Channel>> {
        let span = span!(Level::INFO, "list_channels");
        let _guard = span.enter();
        let url: Url = client
            .url
            .join("/ari/channels")?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to list channels");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not list channels"),
            ));
        }

        event!(Level::INFO, "Successfully received channels");
        let channels = response.json::<Vec<Channel>>().await?;

        Ok(channels)
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
        let span = span!(Level::INFO, "originate_channel");
        let _guard = span.enter();

        let mut url = client.url.join("/ari/channels")?;
        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);

        url.append_pair("endpoint", endpoint);

        event!(Level::INFO, "Originate channel: {}", endpoint);

        if !formats.is_empty() {
            let formats = formats.join(",");
            event!(Level::INFO, "Formats: {}", formats);
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
                    event!(Level::INFO, "App args: {}", app_args);
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        event!(Level::INFO, "Caller ID: {:?}", caller_id);
        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", caller_id);
        }

        event!(Level::INFO, "Timeout: {:?}", timeout);
        if let Some(timeout) = timeout {
            url.append_pair("timeout", &timeout.to_string());
        } else {
            url.append_pair("timeout", "30");
        }

        event!(Level::INFO, "Channel ID: {:?}", channel_id);
        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", channel_id);
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        event!(Level::INFO, "Variables: {:?}", variables);
        let body = json!({
            "variables": variables
        });

        let url = url.finish().to_owned();

        event!(Level::INFO, "URL: {}", url);

        let response = reqwest::Client::new().post(url).json(&body).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to create channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not create channel"),
            ));
        }

        event!(Level::INFO, "Successfully created channel");
        let channel = response.json::<Channel>().await?;

        event!(Level::INFO, "Channel ID: {}", channel.id);
        Ok(channel)
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
        let span = span!(Level::INFO, "create_channel");
        let _guard = span.enter();

        let mut url = client.url.join("/ari/channels")?;
        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);
        url.append_pair("endpoint", endpoint);

        event!(Level::INFO, "Create channel: {}", endpoint);

        if !formats.is_empty() {
            let formats = formats.join(",");
            event!(Level::INFO, "Formats: {}", formats);
            url.append_pair("formats", &formats);
        }

        url.append_pair("app", app);
        if !app_args.is_empty() {
            let app_args = app_args.join(",");
            event!(Level::INFO, "App args: {}", app_args);
            url.append_pair("app_args", &app_args);
        }

        event!(Level::INFO, "Channel ID: {:?}", channel_id);
        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", channel_id);
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        event!(Level::INFO, "Variables: {:?}", variables);
        let body = json!({
            "variables": variables
        });

        let url = url.finish().to_owned();

        event!(Level::INFO, "URL: {}", url);

        let response = reqwest::Client::new().post(url).json(&body).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to create channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not create channel"),
            ));
        }

        event!(Level::INFO, "Successfully created channel");
        let channel = response.json::<Channel>().await?;

        event!(Level::INFO, "Channel ID: {}", channel.id);
        Ok(channel)
    }

    pub async fn get(client: &Client, channel_id: &str) -> Result<Channel> {
        let span = span!(Level::INFO, "get_channel");
        let _guard = span.enter();

        let url = client
            .url
            .join(&format!("/ari/channels/{}", channel_id))?
            .query_pairs_mut()
            .append_pair("api_key", &client.get_api_key())
            .finish()
            .to_owned();

        let response = reqwest::get(url).await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to get channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not get channel"),
            ));
        }

        event!(Level::INFO, "Successfully received channel");
        let channel = response.json::<Channel>().await?;

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
        let span = span!(Level::INFO, "originate_channel_with_id");
        let _guard = span.enter();

        let mut url = client.url.join(&format!("/ari/channels/{}", channel_id))?;
        let mut url = url.query_pairs_mut();

        client.add_api_key(&mut url);
        url.append_pair("endpoint", endpoint);

        if !formats.is_empty() {
            let formats = formats.join(",");
            event!(Level::INFO, "Formats: {}", formats);
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
                    event!(Level::INFO, "App args: {}", app_args);
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        event!(Level::INFO, "Caller ID: {:?}", caller_id);
        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", caller_id);
        }

        event!(Level::INFO, "Timeout: {:?}", timeout);
        if let Some(timeout) = timeout {
            url.append_pair("timeout", &timeout.to_string());
        } else {
            url.append_pair("timeout", "30");
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("otherChannelId", other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", originator);
        }

        event!(Level::INFO, "Variables: {:?}", variables);
        let body = json!({
            "variables": variables
        });

        let url = url.finish().to_owned();

        event!(Level::INFO, "URL: {}", url);

        let response = reqwest::Client::new().post(url).json(&body).send().await?;
        if !response.status().is_success() {
            event!(Level::ERROR, "Failed to create channel");
            return Err(AriError::HttpError(
                response.status(),
                String::from("Could not create channel"),
            ));
        }

        event!(Level::INFO, "Successfully created channel");
        let channel = response.json::<Channel>().await?;

        event!(Level::INFO, "Channel ID: {}", channel.id);
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
