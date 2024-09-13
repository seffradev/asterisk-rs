use arirs::{Client, Event};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).with(LevelFilter::TRACE).init();

    let (request_client, mut event_listener) = Client::connect("http://localhost:8088/", "asterisk", "asterisk", "ari").await?;

    while let Some(event) = event_listener.recv().await {
        if let Event::StasisStart(e) = event {
            let channel = e.channel;
            channel
                .play_media(&request_client, "sound:hello", Some("en"), None, None, None)
                .await?;
        }
    }

    Ok(())
}
