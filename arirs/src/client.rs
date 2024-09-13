use std::time::Duration;

use derive_getters::Getters;
use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{event, Level};
use url::Url;

use crate::*;

#[derive(Debug, Getters)]
pub struct RequestClient {
    url: Url,
    username: String,
    password: String,
}

impl RequestClient {
    pub(crate) fn get_api_key(&self) -> String {
        format!("{}:{}", self.username, self.password)
    }

    pub(crate) fn add_api_key(&self, url: &mut url::form_urlencoded::Serializer<url::UrlQuery>) {
        url.append_pair("api_key", &self.get_api_key());
    }
}

pub struct Client;

impl Client {
    /// Create a new client
    ///
    /// `url` should end in `/`, `ari/` will be appended to it.
    pub async fn connect(
        url: impl AsRef<str>,
        app_name: impl AsRef<str>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<(RequestClient, UnboundedReceiver<Event>)> {
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

        let request_client = RequestClient { url, username, password };

        let event_listener = Self::connect_ws(ws_url).await?;

        Ok((request_client, event_listener))
    }

    async fn connect_ws(ws_url: Url) -> Result<UnboundedReceiver<Event>> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let (ws_stream, _) = connect_async(&ws_url.to_string()).await?;

        let _join_task: JoinHandle<Result<()>> = tokio::task::spawn(async move {
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
                                        if let Err(e) = tx.send(serde_json::from_slice(message.into_data().as_slice()).map_err(|err| AriError::Unknown(err.to_string()))?) {
                                            event!(Level::ERROR, "Error sending event: {}", e);
                                        }
                                    }
                                    tungstenite::Message::Ping(data) => {
                                        event!(Level::TRACE, "Received WebSocket Ping, sending Pong");
                                        ws_sender.send(tungstenite::Message::Pong(data)).await?;
                                    }
                                    tungstenite::Message::Pong(_) => {
                                        event!(Level::TRACE, "Received WebSocket Pong");
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
                                tracing::error!("WebSocket closed");
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
        });

        Ok(rx)
    }
}

impl Default for RequestClient {
    fn default() -> Self {
        Self {
            url: match Url::parse("http://localhost:8088/") {
                Ok(url) => url,
                Err(_) => panic!("Failed to parse URL"),
            },
            username: "asterisk".to_string(),
            password: "asterisk".to_string(),
        }
    }
}
