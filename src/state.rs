use crate::{player::Player, world::Region};

pub struct State {
    regions: Vec<Region>,
    player: Player
}

impl State {
    pub fn new() -> Self {
        Self { regions: Vec::new(), player: Player::new() }
    }
}