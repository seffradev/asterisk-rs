use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::client::{Client, ClientBuilder, Connected};

impl ClientBuilder<Connected> {
    pub fn on_device_state_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(&Client, DeviceStateChanged) + 'static,
    {
        self.data.0.on_device_state_changed = Some(Box::new(f));
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
