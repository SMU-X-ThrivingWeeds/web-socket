use tokio_postgres::{NoTls, Config, Client};

pub async fn connect_to_database() -> Client {
    let (client, connection) = Config::new()
        .user("your_username")
        .password("your_password")
        .host("localhost")
        .port(5432)
        .dbname("your_database")
        .connect(NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(connection);

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
