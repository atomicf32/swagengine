use std::vec;
use bincode::{Decode, Encode};
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Encode, Decode)]
pub enum Block {
    AIR,
    DIRT,
    GRASS,
    STONE
}

impl Block{
    pub fn rand() -> Self {
        let mut rng = rng();
        match rng.random_range(0..4) {
            0 => Block::AIR,
            1 => Block::DIRT,
            2 => Block::GRASS,
            _ => Block::STONE,
        }
    }
}

#[derive(Clone, Encode, Decode)]
pub struct Chunk {
    blocks: Vec<Block> //Chunk size = blocks 131072
}

impl Chunk {
    pub fn rand() -> Self {
        let mut block_vec = Vec::with_capacity(131072);
        for _ in 0..block_vec.capacity() {
            block_vec.push(Block::rand());
        } 

        Self {
             blocks: block_vec
        }
    }

    pub fn blank() -> Self {
        Self {
            blocks: vec![Block::AIR; 131072]
        }
    }

    pub fn get_block(&self, (x, y, z): (i32, i32, i32)) -> Option<&Block> {
        let index = (x + y * 16 + z * 16 * 512) as usize;
        self.blocks.get(index)
    }
}

#[derive(Encode, Decode)]
pub struct Region {
    chunks: Vec<Chunk>, //Region size = 25 chunks = 3276800 blocks
    position: (i32, i32)
}

impl Region {
    pub fn new(position: (i32, i32)) -> Self {
        let chunks = vec![Chunk::rand(); 25]; //Regionis 5 * 5 chunks
        Self {
            chunks: chunks,
            position,
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn get_chunk(&self, (x, y): (i32, i32)) -> Option<&Chunk> {
        let index = (x + y * 5) as usize;
        self.chunks.get(index)
    }
}

pub struct World {
} 