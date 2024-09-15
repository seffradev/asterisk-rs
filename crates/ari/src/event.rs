use chrono::{DateTime, Utc};
use derive_getters::Getters;
use derive_more::derive::Deref;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum AsteriskEvent {
    StasisStart(Box<Event<StasisStart>>),
    StasisEnd(Event<StasisEnd>),
    ChannelCreated(Event<ChannelCreated>),
    ChannelDestroyed(Event<ChannelDestroyed>),
    ChannelVarset(Event<ChannelVarset>),
    ChannelHangupRequest(Event<ChannelHangupRequest>),
    ChannelDialplan(Event<ChannelDialplan>),
    ChannelStateChange(Event<ChannelStateChange>),
    ChannelDtmfReceived(Event<ChannelDtmfReceived>),
    DeviceStateChanged(Event<DeviceStateChanged>),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Getters, Deref)]
#[serde(rename_all = "snake_case")]
pub struct Event<D> {
    asterisk_id: String,
    application: String,
    timestamp: DateTime<Utc>,
    #[deref]
    #[getter(skip)]
    #[serde(flatten)]
    data: D,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    args: Vec<String>,
    channel: Channel,
    replace_channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    cause: i32,
    cause_txt: String,
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    variable: String,
    value: String,
    channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    soft: Option<bool>,
    cause: i32,
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    dialplan_app: String,
    dialplan_app_data: String,
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceStateChanged {
    device_state: DeviceState,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelStateChange {
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    digit: String,
    duration_ms: i32,
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceState {
    name: String,
    state: String,
}
