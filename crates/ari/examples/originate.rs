use std::collections::HashMap;

use asterisk_rs_ari::{AriClient, AriClientError, OriginateChannelParams, OriginateParams};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const APP_NAME: &str = "ari";

#[tokio::main]
async fn main() -> Result<(), AriClientError> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let client = AriClient::new("http://localhost:8088".parse().unwrap(), "asterisk", "asterisk");

    let originate_params = OriginateChannelParams {
        endpoint: "PJSIP/1000",
        params: OriginateParams::Application {
            app: APP_NAME,
            app_args: &[],
        },
        caller_id: None,
        timeout: None,
        channel_id: None,
        other_channel_id: None,
        originator: None,
        formats: &["alaw,ulaw"],
    };

    client.channel_originate(originate_params, &HashMap::new()).await?;

    Ok(())
}
