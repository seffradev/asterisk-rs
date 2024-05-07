use arirs::{client::Client, Event};
use std::sync::{Arc, Mutex};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let dtmf_buffer = Arc::new(Mutex::new(String::new()));

    let client = Client::new()
        .url("http://localhost:8088")?
        .username("asterisk")
        .password("asterisk")
        .app_name("ari")
        .connect()?
        .handler(tx)
        .build()?;

    tokio::spawn(async move {
        if let Err(e) = client.run().await {
            eprintln!("Error: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        match event {
            Event::StasisStart(_) => {
                debug!("Stasis started, resetting DTMF buffer");
                dtmf_buffer.lock().unwrap().clear();
            }
            Event::StasisEnd(_) => {
                debug!("Stasis ended, DTMF buffer: {}", dtmf_buffer.lock().unwrap());
            }
            Event::ChannelDtmfReceived(event) => {
                debug!("Received DTMF: {}", event.digit);
                dtmf_buffer.lock().unwrap().push_str(&event.digit);
            }
            _ => {
                debug!("Unhandled event: {:?}", event);
            }
        }
    }

    Ok(())
}
