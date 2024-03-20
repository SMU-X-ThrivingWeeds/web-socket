use tokio_postgres::{NoTls, Config};

pub async fn connect_to_database() -> tokio_postgres::Client {
    let config_str = "";
    
    let (client, connection) = Config::from_str(config_str, NoTls)
        .connect()
        .await
        .expect("Failed to connect to database");
    
    tokio::spawn(connection.map(|res| {
        if let Err(e) = res {
            eprintln!("Connection error: {}", e);
        }
    }));

    client
}
