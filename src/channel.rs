use crate::{
    client::Client, playback::Playback, recording::Recording, rtp_stat::RtpStat, variable::Variable,
};
use crate::{AriError, Result};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Display;
use tracing::{event, span, Level};
use url::Url;

impl Client {
    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        let span = span!(Level::INFO, "list_channels");
        let _guard = span.enter();
        let url: Url = self
            .url
            .join("/ari/channels")?
            .query_pairs_mut()
            .append_pair("api_key", &format!("{}:{}", self.username, self.password))
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

    pub async fn originate_channel<'a>(
        &self,
        endpoint: &str,
        params: OriginateParams<'a>,
        caller_id: Option<&str>,
        timeout: Option<u32>,
        channel_id: Option<&str>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: Option<HashMap<&str, &str>>,
    ) -> Result<Channel> {
        let span = span!(Level::INFO, "originate_channel");
        let _guard = span.enter();

        let mut url = self.url.join("/ari/channels")?;
        let mut url = url.query_pairs_mut();

        url.append_pair("api_key", &format!("{}:{}", self.username, self.password))
            .append_pair("endpoint", &endpoint);

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
                url.append_pair("extension", &extension);
                if let Some(context) = context {
                    url.append_pair("context", &context);
                }
                if let Some(priority) = priority {
                    url.append_pair("priority", &priority.to_string());
                }
                if let Some(label) = label {
                    url.append_pair("label", &label);
                }
            }
            OriginateParams::Application { app, app_args } => {
                url.append_pair("app", &app);
                if !app_args.is_empty() {
                    let app_args = app_args.join(",");
                    event!(Level::INFO, "App args: {}", app_args);
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        event!(Level::INFO, "Caller ID: {:?}", caller_id);
        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", &caller_id);
        }

        event!(Level::INFO, "Timeout: {:?}", timeout);
        if let Some(timeout) = timeout {
            url.append_pair("timeout", &timeout.to_string());
        } else {
            url.append_pair("timeout", "30");
        }

        event!(Level::INFO, "Channel ID: {:?}", channel_id);
        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", &channel_id);
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", &other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", &originator);
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

    pub async fn create_channel(
        &self,
        endpoint: &str,
        app: &str,
        app_args: Option<Vec<&str>>,
        channel_id: Option<&str>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: Option<HashMap<&str, &str>>,
    ) -> Result<Channel> {
        let span = span!(Level::INFO, "create_channel");
        let _guard = span.enter();

        let mut url = self.url.join("/ari/channels")?;
        let mut url = url.query_pairs_mut();

        url.append_pair("api_key", &format!("{}:{}", self.username, self.password))
            .append_pair("endpoint", &endpoint);

        event!(Level::INFO, "Originate channel: {}", endpoint);

        if !formats.is_empty() {
            let formats = formats.join(",");
            event!(Level::INFO, "Formats: {}", formats);
            url.append_pair("formats", &formats);
        }

        url.append_pair("app", &app);
        if let Some(app_args) = app_args {
            if !app_args.is_empty() {
                let app_args = app_args.join(",");
                event!(Level::INFO, "App args: {}", app_args);
                url.append_pair("app_args", &app_args);
            }
        }

        event!(Level::INFO, "Channel ID: {:?}", channel_id);
        if let Some(channel_id) = channel_id {
            url.append_pair("channel_id", &channel_id);
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("other_channel_id", &other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", &originator);
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

    pub async fn get_channel(&self, channel_id: &str) -> Result<Channel> {
        let span = span!(Level::INFO, "get_channel");
        let _guard = span.enter();

        let url = self
            .url
            .join(&format!("/ari/channels/{}", channel_id))?
            .query_pairs_mut()
            .append_pair("api_key", &format!("{}:{}", self.username, self.password))
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

    pub async fn originate_channel_with_id<'a>(
        &self,
        channel_id: &str,
        endpoint: &str,
        params: OriginateParams<'a>,
        caller_id: Option<&str>,
        timeout: Option<u32>,
        other_channel_id: Option<&str>,
        originator: Option<&str>,
        formats: Vec<&str>,
        variables: Option<HashMap<&str, &str>>,
    ) -> Result<Channel> {
        let span = span!(Level::INFO, "originate_channel_with_id");
        let _guard = span.enter();

        let mut url = self.url.join(&format!("/ari/channels/{}", channel_id))?;
        let mut url = url.query_pairs_mut();

        url.append_pair("api_key", &format!("{}:{}", self.username, self.password))
            .append_pair("endpoint", endpoint);

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
                url.append_pair("extension", &extension);
                if let Some(context) = context {
                    url.append_pair("context", &context);
                }
                if let Some(priority) = priority {
                    url.append_pair("priority", &priority.to_string());
                }
                if let Some(label) = label {
                    url.append_pair("label", &label);
                }
            }
            OriginateParams::Application { app, app_args } => {
                url.append_pair("app", &app);
                if !app_args.is_empty() {
                    let app_args = app_args.join(",");
                    event!(Level::INFO, "App args: {}", app_args);
                    url.append_pair("app_args", &app_args);
                }
            }
        }

        event!(Level::INFO, "Caller ID: {:?}", caller_id);
        if let Some(caller_id) = caller_id {
            url.append_pair("callerId", &caller_id);
        }

        event!(Level::INFO, "Timeout: {:?}", timeout);
        if let Some(timeout) = timeout {
            url.append_pair("timeout", &timeout.to_string());
        } else {
            url.append_pair("timeout", "30");
        }

        event!(Level::INFO, "Other Channel ID: {:?}", other_channel_id);
        if let Some(other_channel_id) = other_channel_id {
            url.append_pair("otherChannelId", &other_channel_id);
        }

        event!(Level::INFO, "Originator: {:?}", originator);
        if let Some(originator) = originator {
            url.append_pair("originator", &originator);
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

    pub async fn hangup_channel(&self, channel_id: &str, reason: Reason) -> Result<()> {
        let span = span!(Level::INFO, "hangup_channel");
        let _guard = span.enter();

        let mut url = self.url.join(&format!("/ari/channels/{}", channel_id))?;

        let mut url = url.query_pairs_mut();

        url.append_pair("api_key", &format!("{}:{}", self.username, self.password));

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

    pub fn continue_in_dialplan(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn move_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub async fn answer_channel(&self, channel_id: &str) -> Result<()> {
        let span = span!(Level::INFO, "answer_channel");
        let _guard = span.enter();

        let url = self
            .url
            .join(&format!("/ari/channels/{}/answer", channel_id))?
            .query_pairs_mut()
            .append_pair("api_key", &format!("{}:{}", self.username, self.password))
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

    pub fn ring_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn send_dtmf(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn mute_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn unmute_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn hold_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn unhold_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
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

    pub async fn play_media(
        &self,
        channel_id: &str,
        media: &str,
        lang: Option<&str>,
        offset_ms: Option<u32>,
        skip_ms: Option<u32>,
        playback_id: Option<&str>,
    ) -> Result<Playback> {
        let span = span!(Level::INFO, "play_media");
        let _guard = span.enter();

        let mut url = self
            .url
            .join(&format!("/ari/channels/{}/play", channel_id))?;

        let mut url = url.query_pairs_mut();

        url.append_pair("api_key", &format!("{}:{}", self.username, self.password))
            .append_pair("media", &media);

        if let Some(lang) = lang {
            event!(Level::INFO, "Lang: {}", lang);
            url.append_pair("lang", &lang);
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
            url.append_pair("playback_id", &playback_id);
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

    pub fn play_media_with_id(&self, _channel_id: &str) -> Result<Playback> {
        unimplemented!()
    }

    pub fn record_channel(&self, _channel_id: &str) -> Result<Recording> {
        unimplemented!()
    }

    pub fn get_channel_variable(&self, _channel_id: &str) -> Result<Variable> {
        unimplemented!()
    }

    pub fn set_channel_variable(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn snoop(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn snoop_with_id(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn dial(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn get_rtp_stat(&self, _channel_id: &str) -> Result<RtpStat> {
        unimplemented!()
    }

    pub fn start_external_media(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Reason::Code(code) => format!("{}", code),
            Reason::Normal => String::from("normal"),
            Reason::Busy => String::from("busy"),
            Reason::Congestion => String::from("congestion"),
            Reason::NoAnswer => String::from("no_answer"),
            Reason::Timeout => String::from("timeout"),
            Reason::Rejected => String::from("rejected"),
            Reason::Unallocated => String::from("unallocated"),
            Reason::NormalUnspecified => String::from("normal_unspecified"),
            Reason::NumberIncomplete => String::from("number_incomplete"),
            Reason::CodecMismatch => String::from("codec_mismatch"),
            Reason::Interworking => String::from("interworking"),
            Reason::Failure => String::from("failure"),
            Reason::AnsweredElsewhere => String::from("answered_elsewhere"),
        };

        write!(f, "{}", res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reason {
    Code(u16),
    Normal,
    Busy,
    Congestion,
    NoAnswer,
    Timeout,
    Rejected,
    Unallocated,
    NormalUnspecified,
    NumberIncomplete,
    CodecMismatch,
    Interworking,
    Failure,
    AnsweredElsewhere,
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
