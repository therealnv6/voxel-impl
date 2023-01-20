use bevy::prelude::Resource;

use crate::chunk::Chunk;

use super::{Chunks, DomainChunk};

#[derive(Resource, Default)]
pub struct LoadedChunks {
    chunks: Vec<i32>,
}

impl LoadedChunks {
    pub fn replace(&mut self, chunks: Vec<i32>) {
        self.chunks = chunks;
    }

    pub fn is_chunk_loaded(&self, chunk: &Chunk) -> bool {
        self.is_chunk_id_loaded(&(Chunks::linearize([chunk.world_pos.x, chunk.world_pos.y])))
    }

    pub fn is_chunk_id_loaded(&self, index: &i32) -> bool {
        self.chunks.contains(index)
    }
}
