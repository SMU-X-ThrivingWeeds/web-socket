use tokio_postgres::{Client, NoTls};

pub async fn connect_to_database() -> Client {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname=mydatabase", NoTls)
        .await
        .expect("Failed to connect to database");
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
}

pub async fn fetch_barcodes(client: &Client) -> Vec<String> {
    let mut barcodes = Vec::new();
    let rows = client.query("SELECT barcode FROM Bottles", &[]).await.expect("Failed to query database");
    for row in rows {
        let barcode: String = row.get(0);
        barcodes.push(barcode);
    }
    barcodes
}
