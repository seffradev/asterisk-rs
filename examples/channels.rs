use std::sync::Arc;

use ari_rs::client::Client;
use ari_rs::Result;
use tokio::sync::mpsc;
use tracing::{debug, error, trace};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let (tx, mut rx) = mpsc::channel(1024);

    let client = Arc::new(
        Client::new()
            .url("http://localhost:8088/ari")?
            .username("asterisk")
            .password("asterisk")
            .app_name("ari")
            .connect()
            .handler(tx)
            .build()?,
    );

    let join = tokio::spawn(async move {
        trace!("Starting client");
        if let Err(e) = client.run().await {
            error!("Error running client: {:?}", e);
        }
    });

    while let Some(event) = rx.recv().await {
        trace!("Received event: {:?}", event);
        match event {
            ari_rs::Event::StasisStart(event) => {
                debug!("Channel ID: {}", event.channel.id);
            }
            _ => debug!("Unhandled event: {:?}", event),
        }
    }

    join.await?;

    Ok(())
}
