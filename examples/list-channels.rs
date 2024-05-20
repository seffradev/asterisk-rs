use arirs::Result;
use arirs::{channel::Channel, client::Client};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new()
        .url("http://localhost:8088/")?
        .username("asterisk")
        .password("asterisk")
        .app_name("ari")
        .build()?;

    for channel in Channel::list(&client).await? {
        debug!("Channel ID: {}", channel.id);
    }

    Ok(())
}
