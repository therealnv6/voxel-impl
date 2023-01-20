use bevy::prelude::{Entity, IVec2, Mesh};
use ndshape::{ConstShape, ConstShape2usize, ConstShape3u32};

use rand::Rng;

use crate::terrain::{self, DebugTerrainGenerator};

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
    pub entity: Entity,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        let mut blocks = [0; ChunkShape::SIZE as usize];
        let terrain = terrain::noise::generate_terrain_3d::<ChunkShape, NoiseShape>(
            rand::thread_rng().gen_range(0..=5000),
            1,
            3.512,
            3.351,
            DebugTerrainGenerator,
        );

        assert_eq!(terrain.len(), ChunkShape::SIZE as usize);
        for i in 0..ChunkShape::SIZE {
            let [x, y, z] = ChunkShape::delinearize(i as u32);
            let linearized = ChunkShape::linearize([x, z, y]);

            if (x > 0 && x < X_SIZE_U32) && (y > 0 && y < Y_SIZE_U32) && (z > 0 && z < Z_SIZE_U32) {
                blocks[linearized as usize] = terrain[i as usize];
            }
        }

        Self {
            blocks,
            world_pos: IVec2::new(x, z),
            mesh: None,
            entity: Entity::from_raw(0),
        }
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
