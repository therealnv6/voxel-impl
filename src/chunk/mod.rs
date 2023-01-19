use bevy::prelude::IVec3;
use height_mesh::ndshape::{ConstShape as _, ConstShape2u32};
use ndshape::{ConstShape, ConstShape3u32};

pub mod container;
pub mod meshing;
pub mod plugin;
pub mod voxel;

pub const X_SIZE: usize = 34;
pub const Y_SIZE: usize = 34;
pub const Z_SIZE: usize = 34;

pub const X_SIZE_U32: u32 = 34;
pub const Y_SIZE_U32: u32 = 34;
pub const Z_SIZE_U32: u32 = 34;

pub type ChunkShape = ConstShape3u32<X_SIZE_U32, Y_SIZE_U32, Z_SIZE_U32>;
pub type HeightShape = ConstShape2u32<X_SIZE_U32, Y_SIZE_U32>;

pub const HEIGHT_SIZE: u32 = HeightShape::SIZE;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    blocks: [u8; ChunkShape::SIZE as usize],
    pub world_pos: IVec3,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut blocks = [0; ChunkShape::SIZE as usize];

        for i in 0..ChunkShape::SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);

            if (x > 0 && x < X_SIZE_U32 - 1)
                && (y > 0 && y < Y_SIZE_U32 - 1)
                && (z > 0 && z < Z_SIZE_U32 - 1)
            {
                if y > (Y_SIZE / 2) as u32 {
                    blocks[i as usize] = 1;
                } else {
                    blocks[i as usize] = 2;
                }
            }
        }

        Self {
            blocks,
            world_pos: IVec3::new(x, y, z),
        }
    }

    pub fn set_block(&mut self, positions: [u32; 3], id: u8) {
        self.blocks[ChunkShape::linearize(positions) as usize] = id;
    }

    pub fn get_block(&self, positions: [u32; 3]) -> u8 {
        self.blocks[ChunkShape::linearize(positions) as usize]
    }
}
