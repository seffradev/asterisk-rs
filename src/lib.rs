use chrono::DateTime;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{event, span, Level};

type Result<T> = std::result::Result<T, AriError>;

impl From<url::ParseError> for AriError {
    fn from(err: url::ParseError) -> Self {
        AriError::UrlParseError(err)
    }
}

#[derive(Debug, Display, Error)]
pub enum AriError {
    UrlParseError(url::ParseError),
    UnsupportedScheme,
}

impl ClientBuilder<Disconnected> {
    pub fn url(mut self, url: &str) -> Self {
        self.data.url = url.to_string();
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.data.username = username.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.data.password = password.to_string();
        self
    }

    pub fn connect(self) -> Result<ClientBuilder<Connected>> {
        let span = span!(Level::INFO, "connect");
        let _guard = span.enter();

        event!(Level::INFO, "Validating URL '{}'", self.data.url);
        url::Url::parse(&self.data.url)?;

        Ok(ClientBuilder {
            data: Connected(Client {
                url: self.data.url,
                username: self.data.username,
                password: self.data.password,
                app_name: self.data.app_name,
                ..Default::default()
            }),
        })
    }
}

impl ClientBuilder<Connected> {
    pub fn build(self) -> Result<Client> {
        let span = span!(Level::INFO, "build");
        let _guard = span.enter();

        event!(
            Level::TRACE,
            "Using REST API server with URL '{}'",
            self.data.0.url
        );

        let url = url::Url::parse(&self.data.0.url)?;
        let host = match url.host_str() {
            Some(host) => host,
            None => {
                event!(Level::ERROR, "No host found in URL '{}'", self.data.0.url);
                return Err(url::ParseError::EmptyHost.into());
            }
        };

        event!(Level::TRACE, "Using host '{}'", host);
let port = url.port().unwrap_or(8088);
        event!(Level::TRACE, "Using port {}", port);

        let scheme = match url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => {
                event!(Level::ERROR, "Unsupported scheme '{}'", url.scheme());
                return Err(AriError::UnsupportedScheme);
            }
        };

        let ws_url = format!(
            "{}://{}:{}/ari/events?app={}&api_key={}:{}?subscribeAll=true",
            scheme, host, port, self.data.0.app_name, self.data.0.username, self.data.0.password
        );

        event!(
            Level::INFO,
            "Connecting to WebSocket server with URL '{}'",
            ws_url
        );

        Ok(self.data.0)
    }
}

#[derive(Debug, Default)]
pub struct ClientBuilder<T: State = Disconnected> {
    data: T,
}

pub trait State {}

impl State for Disconnected {}
impl State for Connected {}

#[derive(Default)]
pub struct Disconnected {
    url: String,
    username: String,
    password: String,
    app_name: String,
}

pub struct Connected(Client);

impl Client {
    pub fn new() -> ClientBuilder<Disconnected> {
        ClientBuilder {
            data: Disconnected {
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
pub struct Client {
    pub url: String,
    pub ws_url: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    pub timestamp: DateTime<chrono::Utc>,
    pub args: Vec<String>,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    pub timestamp: DateTime<chrono::Utc>,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    pub timestamp: DateTime<chrono::Utc>,
    pub channel: Option<Channel>,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    pub timestamp: DateTime<chrono::Utc>,
    pub cause: i32,
    pub cause_txt: String,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    pub timestamp: DateTime<chrono::Utc>,
    pub variable: String,
    pub value: String,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    pub timestamp: DateTime<chrono::Utc>,
    pub soft: Option<bool>,
    pub cause: i32,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    pub timestamp: DateTime<chrono::Utc>,
    pub dialplan_app: String,
    pub dialplan_app_data: String,
    pub channel: Channel,
    pub asterisk_id: String,
    pub application: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub state: String,
    pub protocol_id: String,
    pub caller: Caller,
    pub connected: Caller,
    pub accountcode: String,
    pub dialplan: Dialplan,
    pub creationtime: String,
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Caller {
    pub name: String,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Dialplan {
    pub context: String,
    pub exten: String,
    pub priority: i32,
    pub app_name: String,
    pub app_data: String,
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
}
