use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

use crate::*;

pub struct Client;

impl Client {
    /// Create a new Asterisk ARI client
    ///
    /// Spawns a [`tokio::task`] that connects to the Asterisk
    /// WebSocket endpoint to immideatedly listen to incoming [`Event`]
    /// (`crate::Event`)s. These may be listened to by polling the returned
    /// [`tokio::sync::mpsc::UnboundedReceiver`] stream.
    ///
    // IMPROVEMENT: return differentiable errors for the respective cases
    /// Event listener task is aborted if the listener stream is dropped,
    /// or when WebSocket connection toward the Asterisk is dropped, be it
    /// gracefully or unintentionally.
    ///
    /// # Arguments
    ///
    /// - `url` should end in `/`, `ari/` will be appended to it.
    ///
    /// # Panics
    ///
    /// - If called outside a Tokio runtime.
    /// - If if the library fails to deserialize incoming asterisk messages
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

        let request_client = RequestClient::new(url, username, password);

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
                                        let event_result = serde_json::from_slice(message.into_data().as_slice())
                                            .expect("failed to deserialize asterisk event");
                                        if tx.send(event_result).is_err() {
                                            // Assume that tx has been dropped
                                            break;
                                        }
                                    }
                                    tungstenite::Message::Ping(data) => { ws_sender.send(tungstenite::Message::Pong(data)).await?; }
                                    tungstenite::Message::Pong(_) => { },
                                    tungstenite::Message::Close(_frame) => {
                                        break;
                                    },
                                    tungstenite::Message::Frame(_) => {
                                        unreachable!("raw frame not supposed to be received when reading incoming messages")
                                    }
                                    tungstenite::Message::Binary(_) => {
                                        unreachable!("asterisk should send data marked using text payloads")
                                    }
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    _ = interval.tick() => {
                        // every 5 seconds we are sending ping to keep connection alive
                        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
                        let random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();
                        if ws_sender.send(tungstenite::Message::Ping(random_bytes)).await.is_err() {
                            break;
                        }
                    }
                }
            }

            Ok(())
        });

        Ok(rx)
    }
}
