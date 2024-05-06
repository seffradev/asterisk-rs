use ari_rs::client::Client;
use std::sync::{Arc, Mutex};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let dtmf_buffer = Arc::new(Mutex::new(String::new()));
    let stasis_end_dtmf = dtmf_buffer.clone();
    let dtmf_received_buffer = dtmf_buffer.clone();

    let client = Client::new()
        .url("http://localhost:8088")?
        .username("asterisk")
        .password("asterisk")
        .app_name("ari")
        .connect()
        .on_stasis_start(move |_, _| {
            debug!("Resetting the DTMF buffer");
            dtmf_buffer.lock().unwrap().clear();
        })
        .on_stasis_end(move |_, _| {
            debug!("DTMF buffer: {}", stasis_end_dtmf.lock().unwrap());
        })
        .on_channel_dtmf_received(move |_, event| {
            debug!("Adding DTMF digit: {}", event.digit);
            dtmf_received_buffer.lock().unwrap().push_str(&event.digit);
        })
        .build()?;

    client.run().await?;

    Ok(())
}
