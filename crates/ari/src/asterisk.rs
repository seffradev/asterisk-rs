use std::{result::Result, time::Duration};

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use rand::Rng;
use serde::Serialize;
use thiserror::Error;
use tokio::{sync::mpsc::UnboundedReceiver, time::interval};
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

use crate::*;

#[derive(Debug, Error)]
pub enum AsteriskError {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("unsupported scheme, expected 'http' or 'https', found: '{}'", .0)]
    UnsupportedScheme(String),
    #[error("failed to connect")]
    WebSocketConnect(tokio_tungstenite::tungstenite::Error),
}

pub struct Asterisk;

impl Asterisk {
    /// Connect to the Asterisk, return an ARI request client and an WebSocket event stream.
    ///
    /// Spawns a [`tokio::task`] that connects to the Asterisk
    /// WebSocket endpoint to immideatedly listen to incoming [`AsteriskEvent`]
    /// (`crate::Event`)s. These may be listened to by polling the returned
    /// [`tokio::sync::mpsc::UnboundedReceiver`] stream.
    ///
    /// Event listener task is stopped if the listener stream is dropped,
    /// when WebSocket connection toward the Astbrisk is dropped, be it
    /// gracefully or unintentionally. But also when any
    ///
    /// # Panics
    ///
    /// - If called outside a Tokio runtime.
    /// - If if the library fails to deserialize incoming asterisk messages
    pub async fn connect(
        url: impl AsRef<str>,
        app_name: impl AsRef<str>,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<(AriClient, UnboundedReceiver<AsteriskEvent>), AsteriskError> {
        let url = Self::build_base_url(url.as_ref())?;

        let api_key = Authorization::api_key(username.as_ref(), password.as_ref());

        let ws_url = Self::build_ws_url(&url, &api_key, app_name.as_ref())?;

        let request_client = AriClient::new_with_api_key(url, api_key);

        let event_listener = Self::connect_ws(&ws_url).await?;

        Ok((request_client, event_listener))
    }

    pub(crate) fn build_base_url(url_str: &str) -> Result<Url, AsteriskError> {
        // Prevents `ari/` joins to site.com/example/some/path to resolve to `site.com/ari/`
        let url: Url = match url_str.ends_with('/') {
            true => url_str.parse(),
            false => format!("{}/", url_str).parse(),
        }?;

        let url = url.join("ari/")?;

        Ok(url)
    }

    fn build_ws_url(base_url: &Url, api_key: &str, app_name: &str) -> Result<Url, AsteriskError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct AsteriskWebSocketParams<'a> {
            #[serde(rename = "app")]
            app_name: &'a str,
            subscribe_all: bool,
        }

        let mut ws_url = base_url.join("events")?;
        let scheme = match ws_url.scheme() {
            "http" => "ws",
            "https" => "wss",
            other => Err(AsteriskError::UnsupportedScheme(other.to_string()))?,
        };
        ws_url.set_scheme(scheme).expect("invalid url scheme");

        let ws_url = Authorization::build_url(
            &ws_url,
            [],
            api_key,
            AsteriskWebSocketParams {
                app_name,
                subscribe_all: true,
            },
        )?;

        Ok(ws_url)
    }

    async fn connect_ws(ws_url: &Url) -> Result<UnboundedReceiver<AsteriskEvent>, AsteriskError> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let (ws_stream, _) = connect_async(ws_url).await.map_err(AsteriskError::WebSocketConnect)?;

        tokio::task::spawn(async move {
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            let mut interval = interval(Duration::from_millis(5000));

            loop {
                tokio::select! {
                    message_result = ws_receiver.try_next() => {
                        let Ok(Some(message)) = message_result else {
                            // IMPROVEMENT: might be wiser to propagate the
                            // errors to the event consumer
                            break;
                        };

                        match message {
                            tungstenite::Message::Text(_) => {
                                let event_result = serde_json::from_slice(message.into_data().as_slice())
                                    .expect("failed to deserialize asterisk event");
                                if tx.send(event_result).is_err() {
                                    // Assume that tx has been dropped
                                    break;
                                }
                            }
                            tungstenite::Message::Ping(data) => {
                                if ws_sender.send(tungstenite::Message::Pong(data)).await.is_err() {
                                    // IMPROVEMENT: might be wiser to propagate the
                                    // errors to the event consumer
                                    break;
                                }
                             }
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
        });

        Ok(rx)
    }
}
