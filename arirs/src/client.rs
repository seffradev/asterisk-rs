use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::{sync::mpsc::Sender, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{event, Level};
use url::Url;

use crate::*;

#[derive(Debug, Default)]
pub struct ClientBuilder(Client);

impl ClientBuilder {
    pub fn url(mut self, url: Url) -> Self {
        self.0.url = url;
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.0.username = username.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.0.password = password.to_string();
        self
    }

    pub fn app_name(mut self, app_name: &str) -> Self {
        self.0.app_name = app_name.to_string();
        self
    }

    pub fn handler(mut self, tx: Sender<Event>) -> Self {
        self.0.ws_channel = Some(tx);
        self
    }

    pub fn build(self) -> Result<Client> {
        let mut ws_url = self.0.url.join("events")?;

        let scheme = match ws_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => {
                event!(Level::ERROR, "Unsupported scheme '{}'", ws_url.scheme());
                return Err(tungstenite::error::UrlError::UnsupportedUrlScheme.into());
            }
        };

        if ws_url.set_scheme(scheme).is_err() {
            return Err(tungstenite::error::UrlError::UnsupportedUrlScheme.into());
        }

        ws_url
            .query_pairs_mut()
            .append_pair("app", &self.0.app_name)
            .append_pair("api_key", &format!("{}:{}", self.0.username, self.0.password))
            .append_pair("subscribeAll", "true");

        event!(Level::TRACE, "Using REST API server with URL '{}'", self.0.url);

        event!(Level::TRACE, "Using WebSocket server with URL '{}'", ws_url);

        Ok(Client {
            url: self.0.url,
            ws_url,
            username: self.0.username,
            password: self.0.password,
            app_name: self.0.app_name,
            ws_channel: self.0.ws_channel,
        })
    }
}

#[derive(Debug)]
pub struct Client {
    pub url: Url,
    pub ws_url: Url,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub ws_channel: Option<Sender<Event>>,
}

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ClientBuilder {
        ClientBuilder(Client::default())
    }

    pub fn handle_message(&self, message: Vec<u8>) {
        let data = String::from_utf8(message).unwrap();

        event!(Level::TRACE, "Parsing event");

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
        event!(Level::INFO, "Connecting to Asterisk");

        let (ws_stream, _) = match connect_async(&self.ws_url.to_string()).await {
            Ok(stream) => stream,
            Err(e) => {
                event!(Level::ERROR, "Failed to connect to Asterisk: {}", e);
                return Err(e.into());
            }
        };

        event!(Level::INFO, "WebSocket handshake has been successfully completed");

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

    pub(crate) fn get_api_key(&self) -> String {
        format!("{}:{}", self.username, self.password)
    }

    pub(crate) fn add_api_key(&self, url: &mut url::form_urlencoded::Serializer<url::UrlQuery>) {
        url.append_pair("api_key", &self.get_api_key());
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            url: match Url::parse("http://localhost:8088/") {
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
