use std::collections::HashMap;

use arirs::{Channel, Client, OriginateParams, Result};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const APP_NAME: &str = "ari";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new()
        .url(url::Url::parse("http://localhost:8088/ari")?)
        .username("asterisk")
        .password("asterisk")
        .app_name(APP_NAME)
        .build()?;

    Channel::originate(
        &client,
        "PJSIP/1000",
        OriginateParams::Application {
            app: APP_NAME,
            app_args: vec![],
        },
        None,
        None,
        None,
        None,
        None,
        vec!["alaw,ulaw"],
        HashMap::new(),
    )
    .await?;

    Ok(())
}
