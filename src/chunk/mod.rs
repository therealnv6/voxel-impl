use bevy::prelude::IVec3;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};

pub mod container;
pub mod meshing;
pub mod plugin;
pub mod voxel;

pub const X_SIZE: usize = 16;
pub const Y_SIZE: usize = 16;
pub const Z_SIZE: usize = 16;

pub const X_SIZE_U32: u32 = 16;
pub const Y_SIZE_U32: u32 = 16;
pub const Z_SIZE_U32: u32 = 16;

pub type ChunkShape = ConstShape3u32<X_SIZE_U32, Y_SIZE_U32, Z_SIZE_U32>;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    blocks: [[[i32; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize],
    pub world_pos: IVec3,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut blocks = [[[0; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize];
        let [x_size, y_size, z_size] = ChunkShape::ARRAY;

        for x in 0..x_size {
            for y in 0..y_size {
                for z in 0..z_size {
                    blocks[x as usize][y as usize][z as usize] = 1;
                }
            }
        }

        Self {
            blocks,
            world_pos: IVec3::new(x, y, z),
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, id: i32) {
        self.blocks[x as usize][y as usize][z as usize] = id;
    }
}
