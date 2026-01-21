use clap::Parser;
use futures_util::StreamExt;
use axum::{
    Router,
    routing::get,
    extract::ws::{WebSocketUpgrade, Message},
    response::Response
};
use axum_server::tls_rustls::RustlsConfig;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tls: bool
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let app = Router::new().route("/", get(ws));

    let addr = std::net::SocketAddr::new([127, 0, 0, 1].into(), 3001);

    if args.tls {
        let tls = RustlsConfig::from_pem_file("tls.crt", "tls.key").await.unwrap();
        axum_server::bind_rustls(addr, tls).serve(app.into_make_service()).await.unwrap();
    } else {
        axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
    }
}

async fn ws(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(async move |mut socket| {
        while let Some(msg) = socket.next().await {
            if let x @ Message::Text(_) = msg.unwrap() {
                socket.send(x).await.unwrap();
            }
        }
    })
}
