use crate::channel::ChannelCreated;
use crate::channel::ChannelDestroyed;
use crate::channel::ChannelDialplan;
use crate::channel::ChannelHangupRequest;
use crate::channel::ChannelStateChange;
use crate::channel::ChannelVarset;
use crate::channel::StasisEnd;
use crate::channel::StasisStart;
use crate::device::DeviceStateChanged;
use crate::AriError;
use crate::Event;
use crate::Result;
use futures_util::future;
use futures_util::pin_mut;
use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use tracing::{event, span, Level};

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
    pub data: T,
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

        let (ws_stream, _) = match connect_async(&self.url).await {
            Ok(stream) => stream,
            Err(e) => {
                event!(Level::ERROR, "Failed to connect to Asterisk: {}", e);
                return Err(e.into());
            }
        };

        event!(Level::INFO, "WebSocket handshake has been successfully completed");

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
                    event!(Level::TRACE, "StasisStart: {:?}", event);
                    self.on_stasis_start.as_ref().map(|f| f(event));
                }
                Event::StasisEnd(event) => {
                    event!(Level::TRACE, "StasisEnd: {:?}", event);
                    self.on_stasis_end.as_ref().map(|f| f(event));
                }
                Event::ChannelCreated(event) => {
                    event!(Level::TRACE, "ChannelCreated: {:?}", event);
                    self.on_channel_created.as_ref().map(|f| f(event));
                }
                Event::ChannelDestroyed(event) => {
                    event!(Level::TRACE, "ChannelDestroyed: {:?}", event);
                    self.on_channel_destroyed.as_ref().map(|f| f(event));
                }
                Event::ChannelVarset(event) => {
                    event!(Level::TRACE, "ChannelVarset: {:?}", event);
                    self.on_channel_varset.as_ref().map(|f| f(event));
                }
                Event::ChannelHangupRequest(event) => {
                    event!(Level::TRACE, "ChannelHangupRequest: {:?}", event);
                    self.on_channel_hangup_request.as_ref().map(|f| f(event));
                }
                Event::ChannelDialplan(event) => {
                    event!(Level::TRACE, "ChannelDialplan: {:?}", event);
                    self.on_channel_dialplan.as_ref().map(|f| f(event));
                }
                Event::ChannelStateChange(event) => {
                    event!(Level::TRACE, "ChannelStateChange: {:?}", event);
                    self.on_channel_state_change.as_ref().map(|f| f(event));
                }
                Event::DeviceStateChanged(event) => {
                    event!(Level::TRACE, "DeviceStateChanged: {:?}", event);
                    self.on_device_state_changed.as_ref().map(|f| f(event));
                }
            }
        });

        pin_mut!(ws_in, ws_out);
        future::select(ws_in, ws_out).await;

        Ok(())
    }
}

#[derive(Default)]
pub struct Client {
    pub url: String,
    pub ws_url: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub on_stasis_start: Option<Box<dyn Fn(StasisStart) -> ()>>,
    pub on_stasis_end: Option<Box<dyn Fn(StasisEnd) -> ()>>,
    pub on_channel_created: Option<Box<dyn Fn(ChannelCreated) -> ()>>,
    pub on_channel_destroyed: Option<Box<dyn Fn(ChannelDestroyed) -> ()>>,
    pub on_channel_varset: Option<Box<dyn Fn(ChannelVarset) -> ()>>,
    pub on_channel_hangup_request: Option<Box<dyn Fn(ChannelHangupRequest) -> ()>>,
    pub on_channel_dialplan: Option<Box<dyn Fn(ChannelDialplan) -> ()>>,
    pub on_channel_state_change: Option<Box<dyn Fn(ChannelStateChange) -> ()>>,
    pub on_device_state_changed: Option<Box<dyn Fn(DeviceStateChanged) -> ()>>,
}
