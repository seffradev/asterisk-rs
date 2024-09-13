use channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelDtmfReceived, ChannelHangupRequest, ChannelStateChange, ChannelVarset,
    StasisEnd, StasisStart,
};
use device::DeviceStateChanged;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::JoinError;
use tokio_tungstenite::tungstenite;

pub mod bridge;
pub mod channel;
pub mod client;
pub mod device;
pub mod playback;
pub mod recording;
pub mod rtp_statistics;
pub mod variable;

pub type Result<T> = std::result::Result<T, AriError>;

#[derive(Debug, Error)]
pub enum AriError {
    #[error("URL parsing error")]
    UrlParseError(#[from] url::ParseError),
    #[error("WebSocket error")]
    TungsteniteError(#[from] tungstenite::Error),
    #[error("HTTP Request error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Join Error")]
    JoinError(#[from] JoinError),
    #[error("Unknown error occurred: {0}")]
    Unknown(String),
}

impl From<tungstenite::error::UrlError> for AriError {
    fn from(err: tungstenite::error::UrlError) -> Self {
        AriError::TungsteniteError(err.into())
    }
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
    ChannelDtmfReceived(ChannelDtmfReceived),
    DeviceStateChanged(DeviceStateChanged),
    #[serde(other)]
    Unknown,
}
