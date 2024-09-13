use std::time::Duration;

use derive_getters::Getters;
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::{sync::mpsc::UnboundedSender, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{event, Level};
use url::Url;

use crate::*;

#[derive(Debug, Getters)]
pub struct Client {
    url: Url,
    ws_url: Url,
    username: String,
    password: String,
    ws_channel: Option<UnboundedSender<Event>>,
}

impl Client {
    /// Create a new client
    ///
    /// `url` should end in `/`, `ari/` will be appended to it.
    pub fn new(
        url: impl AsRef<str>,
        app_name: impl AsRef<str>,
        username: impl Into<String>,
        password: impl Into<String>,
        event_sender: Option<UnboundedSender<Event>>,
    ) -> Result<Self> {
        let url = Url::parse(url.as_ref())?.join("ari")?;

        let mut ws_url = url.join("events")?;
        let scheme = match ws_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            _ => Err(tungstenite::error::UrlError::UnsupportedUrlScheme)?,
        };

        let username = username.into();
        let password = password.into();

        ws_url.set_scheme(scheme).expect("invalid url scheme");
        ws_url
            .query_pairs_mut()
            .append_pair("app", app_name.as_ref())
            .append_pair("api_key", &format!("{}:{}", username, password))
            .append_pair("subscribeAll", "true");

        Ok(Self {
            url,
            ws_url,
            username,
            password,
            ws_channel: event_sender,
        })
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
            if let Err(e) = tx.send(event) {
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
            ws_channel: None,
        }
    }
}
