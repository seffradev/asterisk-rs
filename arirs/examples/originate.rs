use std::collections::HashMap;

use arirs::{Channel, OriginateParams, RequestClient, Result};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const APP_NAME: &str = "ari";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = RequestClient::default();

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
