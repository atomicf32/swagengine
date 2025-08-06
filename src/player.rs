pub struct Player {
    health: Health,
    hunger: Hunger,
    inventory: Inventory
}

impl Player {
    pub fn new() -> Self {
        Self {
            health: Health {
                hearts: 20 
             
            },
            hunger: Hunger {
                points: 20
            },
            inventory: Inventory {
                slots: vec![Slot {items: vec![Item::DIRT_BLOCK; 64]}; 36]
            }
        }
    }
}

pub struct Health {
    hearts: u8
}

pub struct Hunger {
    points: u8
}

pub struct Inventory {
    slots: Vec<Slot>
}

#[derive(Clone)]
pub struct Slot {
    items: Vec<Item>
}

#[derive(Clone)]
pub enum Item {
    DIRT_BLOCK,
    GRASS_BLOCK,
    STONE_BLOCK
}