use mongodb::{options::ClientOptions, Client};
use std::env;
use pssstt::common::messages::{Message, ServiceID};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set! Use docker-compose!");

    let mut client_options = ClientOptions::parse(uri).await?;

    client_options.app_name = Some("RustApp".to_string());

    let client = Client::with_options(client_options)?;


    let database_names = client.list_database_names(None, None).await?;

    println!("Databases: {:?}", database_names);

    // Create PSSSTT server
    let sid = ServiceID::new("general manager".to_owned(), "generates a team".to_owned());
    let server = pssstt::server::Server::new(sid, "0.0.0.0:24017");

    let handle_fn = |msg: String| {
        let des_msg = serde_json::from_str::<Message>(&msg).unwrap();
        println!("Incoming msg: {:?}", des_msg);
        return String::from("Testing");
    };
    pssstt::server::Server::start(server, handle_fn);
    
    Ok(())
}
