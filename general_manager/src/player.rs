#[derive(Debug, Default)]
pub struct Player {
    name: String,
    team: String,
}

impl Player {
    pub fn new(name: String, team: String) -> Self {
        Self { name, team}
    }
}
