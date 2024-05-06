use std::pin::Pin;

use channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelDtmfReceived, ChannelHangupRequest,
    ChannelStateChange, ChannelVarset, StasisEnd, StasisStart,
};
use client::ClientProps;
use derive_more::Display;
use device::DeviceStateChanged;
use futures_util::Future;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod channel;
pub mod client;
pub mod device;
pub mod playback;
pub mod recording;
pub mod rtp_stat;
pub mod variable;

pub type Result<T> = std::result::Result<T, AriError>;
pub type Handler<T> = Box<dyn Fn(ClientProps, T) -> Pin<Box<dyn Future<Output = ()>>> + 'static>;
pub type HandlerOption<T> = Option<Handler<T>>;

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

#[derive(Debug, Display, Error)]
pub enum AriError {
    UrlParseError(url::ParseError),
    TungsteniteError(tungstenite::Error),
    ReqwestError(reqwest::Error),
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
