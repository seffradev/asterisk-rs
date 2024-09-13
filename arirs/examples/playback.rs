use std::sync::Arc;

use arirs::{Client, Event};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).with(LevelFilter::TRACE).init();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let client = Arc::new(Client::new("http://localhost:8088/", "asterisk", "asterisk", "ari", Some(tx))?);

    let client_clone = client.clone();
    tokio::spawn(async move {
        if let Err(e) = client_clone.run().await {
            error!("Error: {}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        if let Event::StasisStart(e) = event {
            let channel = e.channel;
            channel.play_media(&client, "sound:hello", Some("en"), None, None, None).await?;
        }
    }

    Ok(())
}
