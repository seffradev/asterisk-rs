use std::sync::{Arc, Mutex};

use arirs::{Client, Event};
use tracing::{debug, error};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let (tx, mut rx) = tokio::sync::mpsc::channel(1024);
    let dtmf_buffer = Arc::new(Mutex::new(String::new()));

    let client = Client::new("http://localhost:8088/", "asterisk", "asterisk", "ari", Some(tx))?;

    tokio::spawn(async move {
        if let Err(e) = client.run().await {
            error!("Error: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
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
