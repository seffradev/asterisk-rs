use chrono::DateTime;
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
pub struct DeviceStateChanged {
    application: String,
    timestamp: DateTime<chrono::Utc>,
    device_state: DeviceState,
    asterisk_id: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceState {
    name: String,
    state: String,
}
