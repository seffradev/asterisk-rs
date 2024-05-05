use ari_rs::Client;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::TRACE)
        .init();

    tracing::info!("Hello, world!");

    let client = Client::new()
        .url("http://localhost:8080")
        .username("admin")
        .password("password")
        .connect()?
        .build();

    Ok(())
}
