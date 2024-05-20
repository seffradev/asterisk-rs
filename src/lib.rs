use channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelDtmfReceived, ChannelHangupRequest,
    ChannelStateChange, ChannelVarset, StasisEnd, StasisStart,
};
use device::DeviceStateChanged;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use tokio::task::JoinError;

pub mod channel;
pub mod client;
pub mod device;
pub mod playback;
pub mod recording;
pub mod rtp_stat;
pub mod variable;

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

impl From<reqwest::Error> for AriError {
    fn from(err: reqwest::Error) -> Self {
        AriError::ReqwestError(err)
    }
}

impl From<JoinError> for AriError {
    fn from(err: JoinError) -> Self {
        AriError::JoinError(err)
    }
}

impl From<String> for AriError {
    fn from(err: String) -> Self {
        AriError::Unknown(err)
    }
}

impl Display for AriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AriError::UrlParseError(err) => write!(f, "UrlParseError: {}", err),
            AriError::TungsteniteError(err) => write!(f, "TungsteniteError: {}", err),
            AriError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            AriError::JoinError(err) => write!(f, "JoinError: {}", err),
            AriError::HttpError(status, body) => {
                write!(f, "HttpError: {} - {}", status, body)
            }
            AriError::Unknown(err) => write!(f, "Unknown: {}", err),
        }
    }
}

#[derive(Debug, Error)]
pub enum AriError {
    UrlParseError(url::ParseError),
    TungsteniteError(tungstenite::Error),
    ReqwestError(reqwest::Error),
    JoinError(JoinError),
    HttpError(reqwest::StatusCode, String),
    Unknown(String),
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
