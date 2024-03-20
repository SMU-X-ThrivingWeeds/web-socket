use tokio::sync::{broadcast, RwLock};
use tokio_postgres::Client;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use crate::db; // Import the db module

pub async fn start_websocket_server(listener: tokio::net::TcpListener, tx: broadcast::Sender<Message>, max_capacity: RwLock<i32>, client: Client) {
    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let tx = tx.clone();
        let max_capacity = max_capacity.clone();
        let client = client.clone();
        tokio::spawn(handle_connection(stream, tx, max_capacity, client));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: broadcast::Sender<Message>, max_capacity: RwLock<i32>, client: Client) {
    // Database operations
    let rows = db::fetch_data_from_database(&client).await;
    
    // Send data to clients via WebSocket
    for row in rows {
        let _ = tx.send(Message::Text(row)).await;
    }

    // WebSocket handling code goes here
    // This function is responsible for interacting with WebSocket connections
}
