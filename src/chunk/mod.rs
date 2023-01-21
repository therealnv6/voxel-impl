use bevy::prelude::{Entity, IVec2, Mesh};
use ndshape::{ConstShape, ConstShape2usize, ConstShape3u32};

pub mod container;
pub mod meshing;
pub mod plugin;
pub mod voxel;

pub const X_SIZE: usize = 32;
pub const Y_SIZE: usize = 32;
pub const Z_SIZE: usize = 32;

pub const X_SIZE_U32: u32 = 32;
pub const Y_SIZE_U32: u32 = 32;
pub const Z_SIZE_U32: u32 = 32;

pub type ChunkShape = ConstShape3u32<X_SIZE_U32, Y_SIZE_U32, Z_SIZE_U32>;
pub type NoiseShape = ConstShape2usize<X_SIZE, Y_SIZE>;

pub const HEIGHT_SIZE: usize = NoiseShape::SIZE;

#[derive(Debug, Clone)]
pub struct Chunk {
    blocks: [u8; ChunkShape::SIZE as usize],
    pub world_pos: IVec2,
    pub mesh: Option<Mesh>,
    pub entity: Option<Entity>,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            mesh: None,
            entity: None,
            blocks: [0; ChunkShape::SIZE as usize],
            world_pos: IVec2::new(x, z),
            dirty: true,
        }
    }

    pub fn set_block_domain(&mut self, position: usize, id: u8) {
        self.blocks[position] = id;
    }

    pub fn get_block_domain(&self, position: usize) -> u8 {
        self.blocks[position]
    }

    pub fn set_block(&mut self, positions: [u32; 3], id: u8) {
        self.blocks[ChunkShape::linearize(positions) as usize] = id;
    }

    pub fn get_block(&self, positions: [u32; 3]) -> u8 {
        self.blocks[ChunkShape::linearize(positions) as usize]
    }
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.world_pos == other.world_pos
    }

    fn ne(&self, other: &Self) -> bool {
        self.world_pos != other.world_pos
    }
}
