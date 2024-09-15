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
