use clap::Parser;
use futures_util::{StreamExt, SinkExt};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tls: bool
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let server = match args.tls {
        true  => "wss://127.0.0.1:3001",
        false => "ws://127.0.0.1:3001"
    };

    let (mut sender, mut receiver) = tokio_tungstenite::connect_async(server).await.unwrap().0.split();

    let send = tokio::spawn(async move {
        for _ in 0..1_000 {
            sender.send("abcdef".into()).await.unwrap();
        }
        sender.close().await.unwrap();
        sender.flush().await.unwrap();
    });

    while let Some(x) = receiver.next().await {
        x.unwrap();
    }

    send.await.unwrap();
}
