use bevy::{prelude::Resource, utils::HashSet};

use crate::chunk::Chunk;

use super::{Chunks, DomainChunk};

#[derive(Resource, Default)]
pub struct LoadedChunks {
    chunks: HashSet<i32>,
    to_unload: HashSet<i32>,
}

impl LoadedChunks {
    pub fn replace(&mut self, chunks: HashSet<i32>) {
        self.to_unload = self.chunks.clone();
        self.to_unload.retain(|chunk| chunks.contains(chunk));

        self.chunks = chunks;
    }

    pub fn is_chunk_loaded(&self, chunk: &Chunk) -> bool {
        self.is_chunk_id_loaded(&(Chunks::linearize([chunk.world_pos.x, chunk.world_pos.y])))
    }

    pub fn is_chunk_id_loaded(&self, index: &i32) -> bool {
        self.chunks.contains(index)
    }

    pub fn pull_unload(&mut self) -> HashSet<i32> {
        self.to_unload.drain().collect()
    }
}
