use channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelHangupRequest, ChannelStateChange,
    ChannelVarset, StasisEnd, StasisStart,
};
use derive_more::Display;
use device::DeviceStateChanged;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod channel;
pub mod client;
pub mod device;

pub type Result<T> = std::result::Result<T, AriError>;

impl From<url::ParseError> for AriError {
    fn from(err: url::ParseError) -> Self {
        AriError::UrlParseError(err)
    }
}

impl From<tungstenite::Error> for AriError {
    fn from(err: tungstenite::Error) -> Self {
        AriError::TungsteniteError(err)
    }
}

impl From<tungstenite::error::UrlError> for AriError {
    fn from(err: tungstenite::error::UrlError) -> Self {
        AriError::TungsteniteError(err.into())
    }
}

#[derive(Debug, Display, Error)]
pub enum AriError {
    UrlParseError(url::ParseError),
    TungsteniteError(tungstenite::Error),
}

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
    DeviceStateChanged(DeviceStateChanged),
    #[serde(other)]
    Unknown,
}
