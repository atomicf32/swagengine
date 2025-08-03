use std::array::from_fn;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Encode, Decode)]
pub enum Block {
    STONE,
    DIRT,
    GRASS,
    AIR
}

#[derive(Encode, Decode)]
pub struct Chunk {
    blocks: Box<[[[Block; 16]; 16]; 16]>, //Chunk size = blocks 4096
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: Box::new([[[Block::DIRT; 16]; 16]; 16]),
        } 
    }

    pub fn get_block (&self, position: (usize, usize, usize)) -> Block {
        self.blocks[position.0][position.1][position.2]
    }
}

#[derive(Encode, Decode)]
pub struct Region {
    chunks: Box<[[[Chunk; 32]; 4]; 4]>,
    position: (i32, i32)
}

impl Region {
    pub fn new(position: (i32, i32)) -> Self {
        let chunks = Box::new(
            from_fn(|_x: usize| {
                from_fn( |_y| {
                    from_fn(|_z| Chunk::new())
                })
        }));
        Self {
            chunks: chunks,
            position,
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn get_chunk(&mut self, position: (usize, usize, usize)) -> &mut Chunk {
        &mut self.chunks[position.0][position.1][position.2]
    }
}

#[derive(Serialize, Deserialize)]
pub struct World {
} 