use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::client::{ClientBuilder, Connected};

impl ClientBuilder<Connected> {
    pub fn on_stasis_start<F>(mut self, f: F) -> Self
    where
        F: Fn(StasisStart) -> () + 'static,
    {
        self.data.0.on_stasis_start = Some(Box::new(f));
        self
    }

    pub fn on_stasis_end<F>(mut self, f: F) -> Self
    where
        F: Fn(StasisEnd) -> () + 'static,
    {
        self.data.0.on_stasis_end = Some(Box::new(f));
        self
    }

    pub fn on_channel_created<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelCreated) -> () + 'static,
    {
        self.data.0.on_channel_created = Some(Box::new(f));
        self
    }

    pub fn on_channel_destroyed<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelDestroyed) -> () + 'static,
    {
        self.data.0.on_channel_destroyed = Some(Box::new(f));
        self
    }

    pub fn on_channel_varset<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelVarset) -> () + 'static,
    {
        self.data.0.on_channel_varset = Some(Box::new(f));
        self
    }

    pub fn on_channel_hangup_request<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelHangupRequest) -> () + 'static,
    {
        self.data.0.on_channel_hangup_request = Some(Box::new(f));
        self
    }

    pub fn on_channel_dialplan<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelDialplan) -> () + 'static,
    {
        self.data.0.on_channel_dialplan = Some(Box::new(f));
        self
    }

    pub fn on_channel_state_change<F>(mut self, f: F) -> Self
    where
        F: Fn(ChannelStateChange) -> () + 'static,
    {
        self.data.0.on_channel_state_change = Some(Box::new(f));
        self
    }
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
    pub channel: Channel,
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
