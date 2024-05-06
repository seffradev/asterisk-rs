use crate::{Event, Result};
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
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
    pub fn handler(mut self, tx: Sender<Event>) -> Self {
        self.data.0.ws_channel = Some(tx);
        self
    }

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
                event!(
                    Level::ERROR,
                    "No host found in URL '{}'",
                    self.data.0.url
                );
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
            "{}://{}:{}/ari/events?app={}&api_key={}:{}&subscribeAll=true",
            scheme,
            host,
            port,
            self.data.0.app_name,
            self.data.0.username,
            self.data.0.password
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

    pub fn handle_message(&self, message: Vec<u8>) {
        let data = String::from_utf8(message.to_vec()).unwrap();

        let event: Event = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(e) => {
                event!(Level::ERROR, "Error: {}", e);
                event!(Level::ERROR, "Data: {}", data);
                return;
            }
        };

        event!(Level::TRACE, "Event parsed successfully");

        if let Some(tx) = &self.ws_channel {
            event!(Level::INFO, "Sending event to channel");
            if let Err(e) = tx.try_send(event) {
                event!(Level::ERROR, "Error sending event: {}", e);
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
        let span = span!(Level::INFO, "run");
        let _guard = span.enter();

        event!(Level::INFO, "Connecting to Asterisk");

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
                                    self.handle_message(message.into_data());
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

impl Default for Client {
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
            ws_channel: None,
        }
    }
}

pub struct Client {
    pub url: Url,
    pub ws_url: Url,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub ws_channel: Option<Sender<Event>>,
}
