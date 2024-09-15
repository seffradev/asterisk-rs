use asterisk_rs_ari::{AriClient, AriClientError};
use tracing::debug;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), AriClientError> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = AriClient::new("http://localhost:8088".parse().unwrap(), "asterisk", "asterisk");

    for channel in client.channel_list().await? {
        debug!("Channel ID: {}", channel.id());
    }

    Ok(())
}
