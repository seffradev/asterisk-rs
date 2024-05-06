use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::{client::{Client, ClientBuilder, Connected}, Handler};

impl ClientBuilder<Connected> {
    pub fn on_device_state_changed(mut self, f: Handler<DeviceStateChanged>) -> Self
    {
        self.data.0.handlers.on_device_state_changed = Some(f);
        self
    }
}

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
