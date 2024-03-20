mod db;
mod websocket;

use tokio::sync::broadcast;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Server running on {}", addr);

    let (tx, _) = broadcast::channel(10);
    let max_capacity = RwLock::new(100); // Initial value of max capacity

    // Connect to the PostgreSQL database
    let client = db::connect_to_database().await;

    // Start WebSocket server
    websocket::start_websocket_server(listener, tx, max_capacity, client).await;
}
