use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
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
