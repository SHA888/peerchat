use std::env;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());
    // TCP listener
    let listener = TcpListener::bind(&addr).await.unwrap();

    let (mut tcp_stream, _socket_addr) = listener.accept().await.unwrap();

    let mut buffer = [0_u8; 1024];

    let bytes_read = tcp_stream.read(&mut buffer).await.unwrap();

    tcp_stream.write_all(&buffer[..bytes_read]).await.unwrap();
}