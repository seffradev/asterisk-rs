use crate::channel::ChannelCreated;
use crate::channel::ChannelDestroyed;
use crate::channel::ChannelDialplan;
use crate::channel::ChannelHangupRequest;
use crate::channel::ChannelStateChange;
use crate::channel::ChannelVarset;
use crate::channel::StasisEnd;
use crate::channel::StasisStart;
use crate::device::DeviceStateChanged;
use crate::Event;
use crate::Result;
use futures_util::future;
use futures_util::pin_mut;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tracing::{event, span, Level};
use url::Url;

impl ClientBuilder<Disconnected> {
    pub fn url(mut self, url: &str) -> Result<Self> {
        event!(Level::INFO, "Validating URL '{}'", self.data.url);
        self.data.url = url::Url::parse(url)?;
        Ok(self)
    }

    pub fn username(mut self, username: &str) -> Self {
        self.data.username = username.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.data.password = password.to_string();
        self
    }

    pub fn app_name(mut self, app_name: &str) -> Self {
        self.data.app_name = app_name.to_string();
        self
    }

    pub fn connect(self) -> ClientBuilder<Connected> {
        let span = span!(Level::INFO, "connect");
        let _guard = span.enter();

        ClientBuilder {
            data: Connected(Client {
                url: self.data.url,
                username: self.data.username,
                password: self.data.password,
                app_name: self.data.app_name,
                ..Default::default()
            }),
        }
    }
}

impl ClientBuilder<Connected> {
    pub fn build(mut self) -> Result<Client> {
        let span = span!(Level::INFO, "build");
        let _guard = span.enter();

        event!(
            Level::TRACE,
            "Using REST API server with URL '{}'",
            self.data.0.url
        );

        let host = match self.data.0.url.host_str() {
            Some(host) => host,
            None => {
                event!(Level::ERROR, "No host found in URL '{}'", self.data.0.url);
                return Err(url::ParseError::EmptyHost.into());
            }
        };

        event!(Level::TRACE, "Using host '{}'", host);
        let port = self.data.0.url.port().unwrap_or(8088);
        event!(Level::TRACE, "Using port {}", port);

        let scheme = match self.data.0.url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => {
                event!(
                    Level::ERROR,
                    "Unsupported scheme '{}'",
                    self.data.0.url.scheme()
                );
                return Err(tungstenite::error::UrlError::UnsupportedUrlScheme.into());
            }
        };

        let ws_url = format!(
            "{}://{}:{}/ari/events?app={}&api_key={}:{}?subscribeAll=true",
            scheme, host, port, self.data.0.app_name, self.data.0.username, self.data.0.password
        );

        self.data.0.ws_url = Url::parse(&ws_url)?;

        Ok(self.data.0)
    }
}

#[derive(Debug, Default)]
pub struct ClientBuilder<T: State = Disconnected> {
    pub data: T,
}

pub trait State {}

impl State for Disconnected {}
impl State for Connected {}

impl Default for Disconnected {
    fn default() -> Self {
        Self {
            url: match Url::parse("http://localhost:8088") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            username: "asterisk".to_string(),
            password: "asterisk".to_string(),
            app_name: "ari".to_string(),
        }
    }
}

pub struct Disconnected {
    url: Url,
    username: String,
    password: String,
    app_name: String,
}

pub struct Connected(pub Client);

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ClientBuilder<Disconnected> {
        ClientBuilder {
            data: Disconnected {
                ..Default::default()
            },
        }
    }

    pub async fn run(&self) -> Result<()> {
        let span = span!(Level::INFO, "run");
        let _guard = span.enter();

        event!(Level::INFO, "Connecting to Asterisk");

        let (_, rx) = futures_channel::mpsc::unbounded();

        let (ws_stream, _) = match connect_async(&self.ws_url).await {
            Ok(stream) => stream,
            Err(e) => {
                event!(Level::ERROR, "Failed to connect to Asterisk: {}", e);
                return Err(e.into());
            }
        };

        event!(
            Level::INFO,
            "WebSocket handshake has been successfully completed"
        );

        let (write, read) = ws_stream.split();

        let ws_out = rx.map(Ok).forward(write);
        let ws_in = read.for_each(|message| async {
            let data = message.unwrap().into_data();

            let data = String::from_utf8(data.to_vec()).unwrap();

            let event: Event = match serde_json::from_str(&data) {
                Ok(data) => data,
                Err(e) => {
                    event!(Level::ERROR, "Error: {}", e);
                    event!(Level::ERROR, "Data: {}", data);
                    return;
                }
            };

            match event {
                Event::StasisStart(event) => {
                    if let Some(f) = &self.on_stasis_start {
                        event!(Level::TRACE, "StasisStart: {:?}", event);
                        f(event);
                    }
                }
                Event::StasisEnd(event) => {
                    if let Some(f) = &self.on_stasis_end {
                        event!(Level::TRACE, "StasisEnd: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelCreated(event) => {
                    if let Some(f) = &self.on_channel_created {
                        event!(Level::TRACE, "ChannelCreated: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelDestroyed(event) => {
                    if let Some(f) = &self.on_channel_destroyed {
                        event!(Level::TRACE, "ChannelDestroyed: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelVarset(event) => {
                    if let Some(f) = &self.on_channel_varset {
                        event!(Level::TRACE, "ChannelVarset: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelHangupRequest(event) => {
                    if let Some(f) = &self.on_channel_hangup_request {
                        event!(Level::TRACE, "ChannelHangupRequest: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelDialplan(event) => {
                    if let Some(f) = &self.on_channel_dialplan {
                        event!(Level::TRACE, "ChannelDialplan: {:?}", event);
                        f(event);
                    }
                }
                Event::ChannelStateChange(event) => {
                    if let Some(f) = &self.on_channel_state_change {
                        event!(Level::TRACE, "ChannelStateChange: {:?}", event);
                        f(event);
                    }
                }
                Event::DeviceStateChanged(event) => {
                    if let Some(f) = &self.on_device_state_changed {
                        event!(Level::TRACE, "DeviceStateChanged: {:?}", event);
                        f(event);
                    }
                }
                Event::Unknown => {
                    event!(Level::INFO, "Unknown event: {}", data);
                }
            }
        });

        pin_mut!(ws_in, ws_out);
        future::select(ws_in, ws_out).await;

        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            url: match Url::parse("http://localhost:8088") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            ws_url: match Url::parse("ws://localhost:8088/ari/events?app=ari&api_key=asterisk:asterisk?subscribeAll=true") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            username: "asterisk".to_string(),
            password: "asterisk".to_string(),
            app_name: "ari".to_string(),
            on_stasis_start: None,
            on_stasis_end: None,
            on_channel_created: None,
            on_channel_destroyed: None,
            on_channel_varset: None,
            on_channel_hangup_request: None,
            on_channel_dialplan: None,
            on_channel_state_change: None,
            on_device_state_changed: None,
        }
    }
}

pub struct Client {
    pub url: Url,
    pub ws_url: Url,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub on_stasis_start: Option<Box<dyn Fn(StasisStart)>>,
    pub on_stasis_end: Option<Box<dyn Fn(StasisEnd)>>,
    pub on_channel_created: Option<Box<dyn Fn(ChannelCreated)>>,
    pub on_channel_destroyed: Option<Box<dyn Fn(ChannelDestroyed)>>,
    pub on_channel_varset: Option<Box<dyn Fn(ChannelVarset)>>,
    pub on_channel_hangup_request: Option<Box<dyn Fn(ChannelHangupRequest)>>,
    pub on_channel_dialplan: Option<Box<dyn Fn(ChannelDialplan)>>,
    pub on_channel_state_change: Option<Box<dyn Fn(ChannelStateChange)>>,
    pub on_device_state_changed: Option<Box<dyn Fn(DeviceStateChanged)>>,
}
