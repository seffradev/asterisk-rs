use arirs::client::Client;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new()
        .url("http://localhost:8088")?
        .username("asterisk")
        .password("asterisk")
        .app_name("ari")
        .connect()?
        .build()?;

    client.run().await?;

    Ok(())
}
