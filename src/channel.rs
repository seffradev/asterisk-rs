use crate::Result;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::{
    client::{Client, ClientBuilder, Connected},
    playback::Playback,
    recording::Recording,
    rtp_stat::RtpStat,
    variable::Variable,
};

impl Client {
    pub fn list_channels(&self) -> Result<Vec<Channel>> {
        unimplemented!()
    }

    pub fn originate_channel(&self) -> Result<Channel> {
        unimplemented!()
    }

    pub fn create_channel(&self) -> Result<Channel> {
        unimplemented!()
    }

    pub fn get_channel(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn originate_channel_with_id(&self, _channel_id: &str) -> Result<Channel> {
        unimplemented!()
    }

    pub fn hangup_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn continue_in_dialplan(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn move_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn answer_channel(&self, _channel_id: &str) -> Result<()> {
        unimplemented!()
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

    pub fn play_media(&self, _channel_id: &str) -> Result<Playback> {
        unimplemented!()
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

impl ClientBuilder<Connected> {
    pub fn on_stasis_start<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, StasisStart) + 'static,
    {
        self.data.0.on_stasis_start = Some(Box::new(f));
        self
    }

    pub fn on_stasis_end<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, StasisEnd) + 'static,
    {
        self.data.0.on_stasis_end = Some(Box::new(f));
        self
    }

    pub fn on_channel_created<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelCreated) + 'static,
    {
        self.data.0.on_channel_created = Some(Box::new(f));
        self
    }

    pub fn on_channel_destroyed<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelDestroyed) + 'static,
    {
        self.data.0.on_channel_destroyed = Some(Box::new(f));
        self
    }

    pub fn on_channel_varset<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelVarset) + 'static,
    {
        self.data.0.on_channel_varset = Some(Box::new(f));
        self
    }

    pub fn on_channel_hangup_request<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelHangupRequest) + 'static,
    {
        self.data.0.on_channel_hangup_request = Some(Box::new(f));
        self
    }

    pub fn on_channel_dialplan<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelDialplan) + 'static,
    {
        self.data.0.on_channel_dialplan = Some(Box::new(f));
        self
    }

    pub fn on_channel_state_change<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelStateChange) + 'static,
    {
        self.data.0.on_channel_state_change = Some(Box::new(f));
        self
    }

    pub fn on_channel_dtmf_received<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, ChannelDtmfReceived) + 'static,
    {
        self.data.0.on_channel_dtmf_received = Some(Box::new(f));
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

pub type StasisStartHandler = Option<Box<dyn Fn(&Client, StasisStart)>>;
pub type StasisEndHandler = Option<Box<dyn Fn(&Client, StasisEnd)>>;
pub type ChannelCreatedHandler = Option<Box<dyn Fn(&Client, ChannelCreated)>>;
pub type ChannelDestroyedHandler = Option<Box<dyn Fn(&Client, ChannelDestroyed)>>;
pub type ChannelVarsetHandler = Option<Box<dyn Fn(&Client, ChannelVarset)>>;
pub type ChannelHangupRequestHandler = Option<Box<dyn Fn(&Client, ChannelHangupRequest)>>;
pub type ChannelDialplanHandler = Option<Box<dyn Fn(&Client, ChannelDialplan)>>;
pub type ChannelStateChangeHandler = Option<Box<dyn Fn(&Client, ChannelStateChange)>>;
pub type ChannelDtmfReceivedHandler = Option<Box<dyn Fn(&Client, ChannelDtmfReceived)>>;
