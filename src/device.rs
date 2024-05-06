use crate::client::Client;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DeviceStateChanged {
    pub application: String,
    pub timestamp: DateTime<chrono::Utc>,
    pub device_state: DeviceState,
    pub asterisk_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DeviceState {
    pub name: String,
    pub state: String,
}

pub type DeviceStateChangedHandler = Option<Box<dyn Fn(&Client, DeviceStateChanged)>>;
