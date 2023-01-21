use bevy::{prelude::Resource, utils::HashMap};
use ndshape::ConstShape;
use once_cell::sync::Lazy;
use parking_lot::{RwLock, RwLockWriteGuard};

use self::queue::ChunkUpdateQueue;

use super::{Chunk, ChunkShape, X_SIZE, Z_SIZE};

pub mod loaded;
pub mod queue;

pub static CHUNK_UPDATE_QUEUE: Lazy<RwLock<ChunkUpdateQueue>> =
    Lazy::new(|| RwLock::new(ChunkUpdateQueue::default()));

pub fn get_update_queue<'a>() -> RwLockWriteGuard<'a, ChunkUpdateQueue> {
    CHUNK_UPDATE_QUEUE.write()
}

pub trait DomainChunk<const N: usize> {
    fn get_domain_at(&mut self, dimensions: [i32; N]) -> &Chunk;
    fn get_domain_at_mut(&mut self, dimensions: [i32; N]) -> &mut Chunk;
    fn get_chunk_at(&mut self, dimensions: [i32; N]) -> &Chunk;
    fn get_chunk_at_mut(&mut self, dimensions: [i32; N]) -> &mut Chunk;
    fn linearize_domain(dimensions: [i32; N]) -> i32;
    fn delinearize_domain(id: i32) -> [i32; N];
    fn linearize(dimensions: [i32; N]) -> i32;
    fn delinearize(id: i32) -> [i32; N];
}

#[derive(Resource, Default, Clone)]
pub struct Chunks {
    chunks: HashMap<i32, Chunk>,
}

unsafe impl Send for Chunks {}
unsafe impl Sync for Chunks {}

impl Chunks {
    pub fn reset(&mut self) {
        for chunk in self.chunks.values_mut() {
            chunk.override_blocks([0u8; ChunkShape::SIZE as usize]);
        }
    }
}

impl DomainChunk<2> for Chunks {
    fn linearize_domain([x, z]: [i32; 2]) -> i32 {
        Chunks::linearize([x * X_SIZE as i32, z * Z_SIZE as i32])
    }

    fn delinearize_domain(id: i32) -> [i32; 2] {
        let [x, z] = Chunks::delinearize(id);

        [x / X_SIZE as i32, z / Z_SIZE as i32]
    }

    fn linearize([x, z]: [i32; 2]) -> i32 {
        let (x_size, z_size) = (X_SIZE as i32, Z_SIZE as i32);
        (x / x_size) * 1024 + (z / z_size)
    }

    fn delinearize(id: i32) -> [i32; 2] {
        let x_size = X_SIZE as i32;
        let z_size = Z_SIZE as i32;
        [(id / 1024) * x_size, (id % 1024) * z_size]
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
