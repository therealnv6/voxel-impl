use bevy::{prelude::Resource, utils::HashMap};

use super::{Chunk, X_SIZE, Y_SIZE, Z_SIZE};

#[derive(Resource)]
pub struct Chunks {
    chunks: HashMap<i32, Chunk>,
}

impl Chunks {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_domain_at(&mut self, x_index: i32, y_index: i32, z_index: i32) -> &Chunk {
        let id = ((x_index) << 20) | ((y_index) << 10) | (z_index);

        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new(
            x_index * X_SIZE as i32,
            y_index * Y_SIZE as i32,
            z_index * Z_SIZE as i32,
        );

        self.chunks.insert(id, chunk);
        self.get_domain_at(x_index, y_index, z_index)
    }

    pub fn get_domain_at_mut(&mut self, x_index: i32, y_index: i32, z_index: i32) -> &Chunk {
        let id = ((x_index) << 20) | ((y_index) << 10) | (z_index);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new(
            x_index * X_SIZE as i32,
            y_index * Y_SIZE as i32,
            z_index * Z_SIZE as i32,
        );

        self.chunks.insert(id, chunk);
        self.get_domain_at_mut(x_index, y_index, z_index)
    }

    pub fn get_chunk_at_mut(&mut self, x: i32, y: i32, z: i32) -> &mut Chunk {
        let (x_size, y_size, z_size) = (X_SIZE as i32, Y_SIZE as i32, Z_SIZE as i32);

        let chunk_x = (x / x_size) * x_size;
        let chunk_y = (y / y_size) * y_size;
        let chunk_z = (z / z_size) * z_size;
        let id = ((chunk_x / x_size) << 20)
            | ((chunk_y / y_size) << 10)
            | ((chunk_z / z_size) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new(chunk_x, chunk_y, chunk_z);

        self.chunks.insert(id, chunk);
        self.get_chunk_at_mut(x, y, z)
    }

    pub fn get_chunk_at(&mut self, x: i32, y: i32, z: i32) -> &Chunk {
        let (x_size, y_size, z_size) = (X_SIZE as i32, Y_SIZE as i32, Z_SIZE as i32);

        let chunk_x = (x / x_size) * x_size;
        let chunk_y = (y / y_size) * y_size;
        let chunk_z = (z / z_size) * z_size;
        let id = ((chunk_x / x_size) << 20)
            | ((chunk_y / y_size) << 10)
            | ((chunk_z / z_size) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new(chunk_x, chunk_y, chunk_z);

        self.chunks.insert(id, chunk);
        self.get_domain_at(x, y, z)
    }
}
