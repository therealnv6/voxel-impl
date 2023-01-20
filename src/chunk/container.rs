use super::{Chunk, X_SIZE, Z_SIZE};
use bevy::{prelude::Resource, utils::HashMap};
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::{RwLock, RwLockWriteGuard};

pub trait DomainChunk<const N: usize> {
    fn get_domain_at(&mut self, dimensions: [i32; N]) -> &Chunk;
    fn get_domain_at_mut(&mut self, dimensions: [i32; N]) -> &mut Chunk;
    fn get_chunk_at(&mut self, dimensions: [i32; N]) -> &Chunk;
    fn get_chunk_at_mut(&mut self, dimensions: [i32; N]) -> &mut Chunk;
    fn linearize_domain(dimensions: [i32; N]) -> i32;
    fn linearize(dimensions: [i32; N]) -> i32;
    fn delinearize(id: i32) -> [i32; N];
}

#[derive(Resource)]
pub struct Chunks {
    chunks: HashMap<i32, Chunk>,
}

impl DomainChunk<2> for Chunks {
    fn linearize_domain([x, z]: [i32; 2]) -> i32 {
        Chunks::linearize([x * X_SIZE as i32, z * Z_SIZE as i32])
    }

    fn linearize([x, z]: [i32; 2]) -> i32 {
        let (x_size, z_size) = (X_SIZE as i32, Z_SIZE as i32);

        let chunk_x = (x.div_euclid(x_size)) * x_size;
        let chunk_z = (z.div_euclid(z_size)) * z_size;

        let id = ((chunk_x / x_size) << 10) | ((chunk_z / z_size) as i32 & 1023);

        id
    }

    fn delinearize(id: i32) -> [i32; 2] {
        let x_size = X_SIZE as i32;
        let z_size = Z_SIZE as i32;

        let x = (id >> 10) * x_size - (x_size / 2);
        let z = (id & 1023) * z_size - (z_size / 2);

        [x, z]
    }

    fn get_chunk_at(&mut self, [x, z]: [i32; 2]) -> &Chunk {
        let id = Self::linearize([x, z]);

        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new(x / X_SIZE as i32, z / Z_SIZE as i32);

        self.chunks.insert(id, chunk);
        self.get_chunk_at([x, z])
    }

    fn get_chunk_at_mut(&mut self, [x, z]: [i32; 2]) -> &mut Chunk {
        let id = Self::linearize([x, z]);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new(x as i32, z as i32);

        self.chunks.insert(id, chunk);
        self.get_chunk_at_mut([x, z])
    }

    fn get_domain_at(&mut self, [x, z]: [i32; 2]) -> &Chunk {
        let id = ((x & 1023) << 10) | (z & 1023);

        // println!("{}, {}", x * X_SIZE as i32, z * Z_SIZE as i32);
        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new(x as i32, z as i32);

        self.chunks.insert(id, chunk);
        self.get_domain_at([x, z])
    }

    fn get_domain_at_mut(&mut self, [x, z]: [i32; 2]) -> &mut Chunk {
        let id = ((x & 1023) << 10) | (z & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new(x as i32, z as i32);

        self.chunks.insert(id, chunk);
        self.get_domain_at_mut([x, z])
    }
}

impl Chunks {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }
}

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

pub static CHUNK_UPDATE_QUEUE: Lazy<RwLock<ChunkUpdateQueue>> =
    Lazy::new(|| RwLock::new(ChunkUpdateQueue::default()));

pub fn get_update_queue<'a>() -> RwLockWriteGuard<'a, ChunkUpdateQueue> {
    CHUNK_UPDATE_QUEUE.write()
}

#[derive(Debug, Default)]
pub struct ChunkUpdateQueue {
    chunks: Vec<i32>,
}

impl ChunkUpdateQueue {
    pub fn queue(&mut self, chunk: i32) {
        self.chunks.push(chunk);
    }

    pub fn pull(&mut self) -> Vec<i32> {
        self.chunks.drain(..).collect()
    }

    pub fn has_queue(&self) -> bool {
        !self.chunks.is_empty()
    }
}
