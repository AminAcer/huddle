use player::Player;
use tokio::time::{sleep, Duration};

mod player;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let player = Player::new("Gibbs".to_owned(), "Lions".to_owned());

    loop {
        println!("Player: {:?}", player);
        sleep(Duration::from_millis(100)).await;
    }
}
