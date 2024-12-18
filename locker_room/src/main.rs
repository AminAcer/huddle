use mongodb::{options::ClientOptions, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("HELLO WORLD!");
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set! Use docker-compose!");

    let mut client_options = ClientOptions::parse(uri).await?;

    client_options.app_name = Some("RustApp".to_string());

    let client = Client::with_options(client_options)?;

    let database_names = client.list_database_names(None, None).await?;

    println!("Databases: {:?}", database_names);
    Ok(())
}
