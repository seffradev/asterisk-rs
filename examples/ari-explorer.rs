use futures_util::{future, pin_mut, StreamExt};
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(LevelFilter::ERROR)
        .init();

    let protocol = "ws";
    let host = "localhost";
    let port = 8088;
    let subscribe = false;
    let app_name = "ari";
    let username = "asterisk";
    let password = "asterisk";

    let subscribe = if subscribe { "&subscribeAll" } else { "" };

    let connect_address = format!(
        "{protocol}://{host}:{port}/ari/events?app={app_name}&api_key={username}:{password}{subscribe}",
    );

    let url = url::Url::parse(&connect_address)?;

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();

            let data = String::from_utf8(data.to_vec()).unwrap();

            let event: ari_rs::Event = match serde_json::from_str(&data) {
                Ok(data) => data,
                Err(e) => {
                    println!("Error: {}", e);
                    println!("Data: {}", data);
                    return;
                }
            };

            match event {
                ari_rs::Event::StasisStart(x) => println!("StasisStart, {}", x.timestamp),
                ari_rs::Event::ChannelCreated(x) => println!("ChannelCreated, {}", x.timestamp),
                ari_rs::Event::ChannelDestroyed(x) => println!("ChannelDestroyed, {}", x.timestamp),
                ari_rs::Event::ChannelVarset(x) => println!("ChannelVarset, {}", x.timestamp),
                ari_rs::Event::ChannelHangupRequest(x) => {
                    println!("ChannelHangupRequest, {}", x.timestamp)
                }
                ari_rs::Event::ChannelDialplan(x) => println!("ChannelDialplan, {}", x.timestamp),
                ari_rs::Event::DeviceStateChanged(x) => {
                    println!("DeviceStateChanged, {}", x.timestamp)
                }
                ari_rs::Event::StasisEnd(x) => println!("StasisEnd, {}", x.timestamp),
                ari_rs::Event::ChannelStateChange(x) => {
                    println!("ChannelStateChange, {}", x.timestamp)
                }
            }
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
    Ok(())
}
