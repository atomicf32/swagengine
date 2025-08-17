use crate::{player::Player, world::Region};

pub struct State {
    pub regions: Vec<Region>,
    pub player: Player
}

impl State {
    pub fn new() -> Self {
        Self { regions: Vec::new(), player: Player::new() }
    }
}