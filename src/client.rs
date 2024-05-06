use crate::channel::{
    ChannelCreated, ChannelDestroyed, ChannelDialplan, ChannelDtmfReceived, ChannelHangupRequest,
    ChannelStateChange, ChannelVarset, StasisEnd, StasisStart,
};
use crate::device::DeviceStateChanged;
use crate::{Event, HandlerOption, Result};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
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
            data: Connected(ClientInner {
                props: ClientProps {
                    url: self.data.url,
                    username: self.data.username,
                    password: self.data.password,
                    app_name: self.data.app_name,
                    ..Default::default()
                },
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
            self.data.0.props.url
        );

        let host = match self.data.0.props.url.host_str() {
            Some(host) => host,
            None => {
                event!(Level::ERROR, "No host found in URL '{}'", self.data.0.props.url);
                return Err(url::ParseError::EmptyHost.into());
            }
        };

        event!(Level::TRACE, "Using host '{}'", host);
        let port = self.data.0.props.url.port().unwrap_or(8088);
        event!(Level::TRACE, "Using port {}", port);

        let scheme = match self.data.0.props.url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => {
                event!(
                    Level::ERROR,
                    "Unsupported scheme '{}'",
                    self.data.0.props.url.scheme()
                );
                return Err(tungstenite::error::UrlError::UnsupportedUrlScheme.into());
            }
        };

        let ws_url = format!(
            "{}://{}:{}/ari/events?app={}&api_key={}:{}&subscribeAll=true",
            scheme, host, port, self.data.0.props.app_name, self.data.0.props.username, self.data.0.props.password
        );

        self.data.0.props.ws_url = Url::parse(&ws_url)?;

        Ok(Client(Arc::new(self.data.0)))
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

pub struct Connected(pub ClientInner);

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ClientBuilder<Disconnected> {
        ClientBuilder {
            data: Disconnected {
                ..Default::default()
            },
        }
    }

    pub async fn handle_message(self, message: Vec<u8>) {
        let data = String::from_utf8(message.to_vec()).unwrap();

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
                if let Some(f) = &self.0.handlers.on_stasis_start {
                    event!(Level::TRACE, "StasisStart: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::StasisEnd(event) => {
                if let Some(f) = &self.0.handlers.on_stasis_end {
                    event!(Level::TRACE, "StasisEnd: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelCreated(event) => {
                if let Some(f) = &self.0.handlers.on_channel_created {
                    event!(Level::TRACE, "ChannelCreated: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelDestroyed(event) => {
                if let Some(f) = &self.0.handlers.on_channel_destroyed {
                    event!(Level::TRACE, "ChannelDestroyed: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelVarset(event) => {
                if let Some(f) = &self.0.handlers.on_channel_varset {
                    event!(Level::TRACE, "ChannelVarset: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelHangupRequest(event) => {
                if let Some(f) = &self.0.handlers.on_channel_hangup_request {
                    event!(Level::TRACE, "ChannelHangupRequest: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelDialplan(event) => {
                if let Some(f) = &self.0.handlers.on_channel_dialplan {
                    event!(Level::TRACE, "ChannelDialplan: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelStateChange(event) => {
                if let Some(f) = &self.0.handlers.on_channel_state_change {
                    event!(Level::TRACE, "ChannelStateChange: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::ChannelDtmfReceived(event) => {
                if let Some(f) = &self.0.handlers.on_channel_dtmf_received {
                    event!(Level::TRACE, "ChannelDtmfReceived: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::DeviceStateChanged(event) => {
                if let Some(f) = &self.0.handlers.on_device_state_changed {
                    event!(Level::TRACE, "DeviceStateChanged: {:?}", event);
                    f(self.0.props, event).await;
                }
            }
            Event::Unknown => {
                event!(Level::INFO, "Unknown event: {}", data);
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        let span = span!(Level::INFO, "run");
        let _guard = span.enter();

        event!(Level::INFO, "Connecting to Asterisk");

        let (ws_stream, _) = match connect_async(&self.0.props.ws_url).await {
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

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let mut interval = interval(Duration::from_millis(5000));

        loop {
            tokio::select! {
                message = ws_receiver.next() => {
                    match message {
                        Some(message) => {
                            let message = message?;
                            match message {
                                tungstenite::Message::Text(_) => {
                                    event!(Level::INFO, "Received WebSocket Text");
                                    self.clone().handle_message(message.into_data());
                                }
                                tungstenite::Message::Ping(data) => {
                                    event!(Level::INFO, "Received WebSocket Ping, sending Pong");
                                    ws_sender.send(tungstenite::Message::Pong(data)).await?;
                                }
                                tungstenite::Message::Pong(_) => {
                                    event!(Level::INFO, "Received WebSocket Pong");
                                },
                                tungstenite::Message::Close(frame) => {
                                    event!(Level::INFO, "WebSocket closed: {:?}", frame);
                                    break;
                                },
                                _ => {
                                    event!(Level::INFO, "Unknown WebSocket message");
                                }
                            }
                        }
                        None => {
                            event!(Level::INFO, "WebSocket closed");
                            break;
                        }
                    }
                }
                _ = interval.tick() => {
                    // every 5 seconds we are sending ping to keep connection alive
                    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
                    let random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
                    let _ = ws_sender.send(tungstenite::Message::Ping(random_bytes)).await;
                    event!(Level::DEBUG, "ARI connection ping sent");
                }
            }
        }

        Ok(())
    }
}

impl Default for ClientProps {
    fn default() -> Self {
        Self {
            url: match Url::parse("http://localhost:8088/ari") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            ws_url: match Url::parse("ws://localhost:8088/ari/events?app=ari&api_key=asterisk:asterisk&subscribeAll=true") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            username: "asterisk".to_string(),
            password: "asterisk".to_string(),
            app_name: "ari".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Client(Arc<ClientInner>);

#[derive(Default)]
pub struct ClientInner {
    pub props: ClientProps,
    pub handlers: ClientHandlers,
}

pub struct ClientProps {
    pub url: Url,
    pub ws_url: Url,
    pub username: String,
    pub password: String,
    pub app_name: String,
}

#[derive(Default)]
pub struct ClientHandlers {
    pub on_stasis_start: HandlerOption<StasisStart>,
    pub on_stasis_end: HandlerOption<StasisEnd>,
    pub on_channel_created: HandlerOption<ChannelCreated>,
    pub on_channel_destroyed: HandlerOption<ChannelDestroyed>,
    pub on_channel_varset: HandlerOption<ChannelVarset>,
    pub on_channel_hangup_request: HandlerOption<ChannelHangupRequest>,
    pub on_channel_dialplan: HandlerOption<ChannelDialplan>,
    pub on_channel_state_change: HandlerOption<ChannelStateChange>,
    pub on_channel_dtmf_received: HandlerOption<ChannelDtmfReceived>,
    pub on_device_state_changed: HandlerOption<DeviceStateChanged>,
}
