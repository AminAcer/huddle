use tokio::time::{sleep, Duration};
use pssstt::common::messages::ServiceID;

mod player;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sid = ServiceID::new("general manager".to_owned(), "generates a team".to_owned());
    let client = pssstt::client::Client::new(sid, "locker_room:24017");
    loop {
        client.send_message("Give me something".to_owned());
        sleep(Duration::from_millis(100)).await;
    }
}
