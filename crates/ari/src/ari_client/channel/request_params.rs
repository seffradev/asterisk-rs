use serde::{ser::SerializeMap, Serialize, Serializer};
use strum::AsRefStr;

#[derive(Debug, Serialize)]
#[serde(tag = "direction", rename_all = "camelCase")]
pub enum Direction {
    In,
    Out,
    Both,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginateChannelParams<'a> {
    pub endpoint: &'a str,
    pub params: OriginateParams<'a>,
    pub caller_id: Option<&'a str>,
    pub timeout: Option<u32>,
    pub channel_id: Option<&'a str>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginateChannelWithIdParams<'a> {
    pub endpoint: &'a str,
    pub params: OriginateParams<'a>,
    pub caller_id: Option<&'a str>,
    pub timeout: Option<u32>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum OriginateParams<'a> {
    Extension {
        extension: &'a str,
        context: Option<&'a str>,
        priority: Option<i32>,
        label: Option<&'a str>,
    },
    Application {
        app: &'a str,
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        #[serde(serialize_with = "join_serialize")]
        app_args: &'a [&'a str],
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendDtmfParams<'a> {
    pub dtmf: &'a str,
    /// in milliseconds
    pub between: Option<u32>,
    /// in milliseconds
    pub duration: Option<u32>,
    pub before: Option<u32>,
    pub after: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayMediaParams<'a> {
    pub playback_id: Option<&'a str>,
    #[serde(flatten)]
    pub base_params: PlayMediaBaseParams<'a>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayMediaBaseParams<'a> {
    #[serde(serialize_with = "join_serialize")]
    pub media: &'a [&'a str],
    pub lang: Option<&'a str>,
    #[serde(rename = "offsetms")]
    pub offset_ms: Option<u32>,
    #[serde(rename = "skipms")]
    pub skip_ms: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordParams<'a> {
    pub name: &'a str,
    pub format: &'a str,
    pub max_duration_seconds: Option<u32>,
    pub max_silence_seconds: Option<u32>,
    pub if_exists: RecordingAction,
    pub beep: bool,
    pub terminate_on: RecordingTermination,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingAction {
    Overwrite,
    Append,
    Fail,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordingTermination {
    None,
    Any,
    #[serde(rename = "*")]
    Asterisk,
    #[serde(rename = "#")]
    Octothorpe,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DialParams<'a> {
    pub caller: Option<&'a str>,
    pub timeout: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelCreateParams<'a> {
    pub endpoint: &'a str,
    pub app: &'a str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub app_args: &'a [&'a str],
    pub channel_id: Option<&'a str>,
    pub other_channel_id: Option<&'a str>,
    pub originator: Option<&'a str>,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    #[serde(serialize_with = "join_serialize")]
    pub formats: &'a [&'a str],
}

fn join_serialize<S>(slice: &[&str], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&slice.join(","))
}

// NOTE: camelCase exception
#[derive(Debug, AsRefStr)]
pub enum Reason {
    Code(u16),
    #[strum(serialize = "normal")]
    Normal,
    #[strum(serialize = "busy")]
    Busy,
    #[strum(serialize = "congestion")]
    Congestion,
    #[strum(serialize = "no_answer")]
    NoAnswer,
    #[strum(serialize = "timeout")]
    Timeout,
    #[strum(serialize = "rejected")]
    Rejected,
    #[strum(serialize = "unallocated")]
    Unallocated,
    #[strum(serialize = "normal_unspecified")]
    NormalUnspecified,
    #[strum(serialize = "number_incomplete")]
    NumberIncomplete,
    #[strum(serialize = "codec_mismatch")]
    CodecMismatch,
    #[strum(serialize = "interworking")]
    Interworking,
    #[strum(serialize = "failure")]
    Failure,
    #[strum(serialize = "answered_elsewhere")]
    AnsweredElsewhere,
}

impl Serialize for Reason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Reason::Code(code) => map.serialize_entry("reason_code", code)?,
            _ => map.serialize_entry("reason", self.as_ref())?,
        };
        map.end()
    }
}
