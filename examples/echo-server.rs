use std::{env, io::Error};

use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tungstenite::{Message, Result};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    let mut barcodes: Vec<String> = Vec::new();
    let mut is_full = "0"; // Default is 0, indicating not full

    loop {
        if let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) if text.trim() == "done" => {
                    // Keep is_full as "0"
                    break;
                }
                Message::Text(text) if text.trim() == "full" => {
                    is_full = "1"; // Change is_full to "1", indicating full
                    break;
                }
                Message::Text(text) => {
                    // Add barcode to the list
                    barcodes.push(text.trim().to_string());
                }
                _ => {
                    // Ignore other message types
                }
            }
        }
    }

    // Format the URL with barcodes and is_full flag
    let barcodes_str = format!("[{}]", barcodes.join(", "));
    let response = format!("http://localhost/gettertesting/receivehere.html?scanlist={}&isfull={}", barcodes_str, is_full);

    // Send back the formatted URL
    if let Err(e) = write.send(Message::Text(response)).await {
        eprintln!("Failed to send message: {}", e);
        return;
    }
}
