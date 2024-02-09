#[derive(
    Default,
    // sane defaults for unit-like enums:
    Clone,
    Copy,
    derive_more::Display,
    Debug,
    // hashable:
    PartialEq,
    Eq,
    Hash,
)]
pub enum Player {
    #[default] // default player to first take action
    P1,
    P2,
}

impl Player {
    pub fn new() -> Player {
        Player::default()
    }

    pub fn next(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}
