use std::sync::Arc;
use tokio_tungstenite::tungstenite;
use tungstenite::Message;

use tokio::net::TcpListener;
use tokio_rustls::rustls;
use tokio_rustls::server::TlsAcceptor;
use rustls_pki_types::{PrivateKeyDer, CertificateDer, pem::PemObject};
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let cert = CertificateDer::pem_file_iter("tls.crt").unwrap().map(Result::unwrap).collect::<Vec<_>>();
    let key = PrivateKeyDer::from_pem_file("tls.key").unwrap();
    let server_config = rustls::server::ServerConfig::builder().with_no_client_auth().with_single_cert(cert, key).unwrap();

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    let acceptor = TlsAcceptor::from(Arc::new(server_config));
    loop {
        let mut socket = tokio_tungstenite::accept_async(acceptor.accept(listener.accept().await.unwrap().0).await.unwrap()).await.unwrap();

        tokio::spawn(async move {
            while let Some(message) = socket.next().await {
                if let x @ Message::Text(_) = message.unwrap() {
                    socket.send(x).await.unwrap();
                }
            }
        });
    }
}
