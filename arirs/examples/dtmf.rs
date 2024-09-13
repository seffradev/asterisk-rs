use std::sync::{Arc, Mutex};

use arirs::{Client, Event};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let dtmf_buffer = Arc::new(Mutex::new(String::new()));

    let (_, mut event_listener) = Client::connect("http://localhost:8088/", "asterisk", "asterisk", "ari").await?;

    while let Some(event) = event_listener.recv().await {
        match event {
            Event::ChannelDtmfReceived(event) => {
                debug!("Received DTMF: {}", event.digit);
                dtmf_buffer.lock().unwrap().push_str(&event.digit);
            }
            Event::StasisEnd(_) => {
                debug!("Stasis ended, DTMF buffer: {}", dtmf_buffer.lock().unwrap());
                dtmf_buffer.lock().unwrap().clear();
            }
            _ => {}
        }
    }

    Ok(())
}
