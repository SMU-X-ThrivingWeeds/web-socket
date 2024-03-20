use tokio_postgres::{NoTls, Config};

pub async fn connect_to_database() -> tokio_postgres::Client {
    let config_str = "postgres://postgres.wovuoddicfjlfzrysyuy:5%tu82@cFmH8KH*@aws-0-ap-southeast-1.pooler.supabase.com:5432/postgres";
    
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
