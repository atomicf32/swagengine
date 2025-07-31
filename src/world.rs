use glam::USizeVec3;

#[derive(Clone, Copy)]
pub enum Block {
    STONE,
    DIRT,
    GRASS
}

pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    pub fn new() -> Self {
        Self { blocks: [[[Block::STONE; 16]; 16]; 16] }
    }
}
