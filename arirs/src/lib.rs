use channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelDtmfReceived, ChannelHangupRequest, ChannelStateChange, ChannelVarset,
    StasisEnd, StasisStart,
};
use device::DeviceStateChanged;
use serde::{Deserialize, Serialize};

pub mod bridge;
pub mod channel;
pub mod client;
pub mod device;
pub mod playback;
pub mod recording;
pub mod rtp_statistics;
pub mod variable;

mod error;
pub use error::{AriError, Result};

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
