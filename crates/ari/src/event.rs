use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    StasisStart(StasisStart),
    StasisEnd(StasisEnd),
    ChannelCreated(ChannelCreated),
    ChannelDestroyed(ChannelDestroyed),
    ChannelVarset(ChannelVarset),
    ChannelHangupRequest(ChannelHangupRequest),
    ChannelDialplan(ChannelDialplan),
    ChannelStateChange(ChannelStateChange),
    ChannelDtmfReceived(ChannelDtmfReceived),
    DeviceStateChanged(DeviceStateChanged),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    timestamp: DateTime<Utc>,
    args: Vec<String>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    timestamp: DateTime<Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    timestamp: DateTime<Utc>,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    timestamp: DateTime<Utc>,
    cause: i32,
    cause_txt: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    timestamp: DateTime<Utc>,
    variable: String,
    value: String,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    timestamp: DateTime<Utc>,
    soft: Option<bool>,
    cause: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    timestamp: DateTime<Utc>,
    dialplan_app: String,
    dialplan_app_data: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceStateChanged {
    application: String,
    timestamp: DateTime<Utc>,
    device_state: DeviceState,
    asterisk_id: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelStateChange {
    timestamp: DateTime<Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    timestamp: DateTime<Utc>,
    digit: String,
    duration_ms: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceState {
    name: String,
    state: String,
}
