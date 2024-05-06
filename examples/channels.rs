use ari_rs::client::Client;
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new()
        .url("http://localhost:8088/ari")?
        .username("asterisk")
        .password("asterisk")
        .app_name("ari")
        .connect()
        .on_stasis_start(|client, _| Box::new(async {
            if let Ok(channels) = client.list_channels().await {
                for channel in channels {
                    debug!("Channel: {:?}", channel);
                }
            }
        }))
        .build()?;

    client.run().await?;

    Ok(())
}
