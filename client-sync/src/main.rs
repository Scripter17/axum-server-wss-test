use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tls: bool
}

fn main() {
    let args = Args::parse();

    let server = match args.tls {
        true  => "wss://127.0.0.1:3001",
        false => "ws://127.0.0.1:3001"
    };

    let mut socket = tungstenite::connect(server).unwrap().0;

    for _ in 0..1000 {
        socket.send("abcdef".into()).unwrap();
        socket.read().unwrap();
    }

    socket.close(None).unwrap();
    socket.read().unwrap();

    assert!(matches!(socket.read(), Err(tungstenite::error::Error::ConnectionClosed)));
}
