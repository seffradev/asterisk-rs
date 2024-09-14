use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Restart,
    Pause,
    Unpause,
    Reverse,
    Forward,
}
