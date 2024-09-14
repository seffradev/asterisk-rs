use chrono::DateTime;
use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Channel {
    id: String,
    name: String,
    state: String,
    protocol_id: String,
    caller: Caller,
    connected: Caller,
    accountcode: String,
    dialplan: Dialplan,
    creationtime: String,
    language: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct RtpStatistics {
    id: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVariable {
    id: String,
}
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    timestamp: DateTime<chrono::Utc>,
    args: Vec<String>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    timestamp: DateTime<chrono::Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    timestamp: DateTime<chrono::Utc>,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    timestamp: DateTime<chrono::Utc>,
    cause: i32,
    cause_txt: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    timestamp: DateTime<chrono::Utc>,
    variable: String,
    value: String,
    channel: Option<Channel>,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    timestamp: DateTime<chrono::Utc>,
    soft: Option<bool>,
    cause: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    timestamp: DateTime<chrono::Utc>,
    dialplan_app: String,
    dialplan_app_data: String,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelStateChange {
    timestamp: DateTime<chrono::Utc>,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    timestamp: DateTime<chrono::Utc>,
    digit: String,
    duration_ms: i32,
    channel: Channel,
    asterisk_id: String,
    application: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Caller {
    name: String,
    number: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Dialplan {
    context: String,
    exten: String,
    priority: i32,
    app_name: String,
    app_data: String,
}
