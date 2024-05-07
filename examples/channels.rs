use std::sync::Arc;

use arirs::client::Client;
use arirs::Result;
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

    let client_clone = Arc::clone(&client);

    let join = tokio::spawn(async move {
        trace!("Starting client");
        if let Err(e) = client.run().await {
            error!("Error running client: {:?}", e);
        }
    });

    if let Ok(channel) = client_clone.originate_channel("PJSIP/1000".to_string(), None, None, None, None, None, None, vec!["ulaw".to_string()], None).await {
        debug!("Channel ID: {}", channel.id);
    } else {
        error!("Error originating channel");
    }

    while let Some(event) = rx.recv().await {
        match event {
            arirs::Event::StasisStart(event) => {
                debug!("Channel ID: {}", event.channel.id);
                if let Ok(channels) = client_clone.list_channels().await {
                    for channel in channels{
                        debug!("Channel ID: {}", channel.id);
                    }
                }
            }
            _ => debug!("Ignore unhandled event"),
        }
    }

    join.await?;

    Ok(())
}
