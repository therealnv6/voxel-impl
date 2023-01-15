use bevy::{
    prelude::{
        shape::Cube, Assets, Commands, Mesh, PbrBundle, Plugin, Res, ResMut, Resource,
        StandardMaterial, Transform, Vec3,
    },
    time::Time,
    utils::HashMap,
};
use bevy_rapier3d::na::coordinates::X;
use noise::{Fbm, NoiseFn, Perlin};

use crate::material::Materials;

pub const X_SIZE: i32 = 16;
pub const Y_SIZE: i32 = 5;
pub const Z_SIZE: i32 = 16;

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

    pub fn get_chunk_at_mut(&mut self, x: i32, y: i32, z: i32) -> &mut Chunk {
        let id = ((x / X_SIZE) << 20) | ((y / Y_SIZE) << 10) | ((z / Z_SIZE) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new();

        self.chunks.insert(id, chunk);
        self.get_chunk_at_mut(x, y, z)
    }

    pub fn get_chunk_at(&mut self, x: i32, y: i32, z: i32) -> &Chunk {
        let id = ((x / X_SIZE) << 20) | ((y / Y_SIZE) << 10) | ((z / Z_SIZE) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new();

        self.chunks.insert(id, chunk);
        self.get_chunk_at(x, y, z)
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    blocks: [[[i32; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize],
}

impl Chunk {
    pub fn new() -> Self {
        let mut blocks = [[[0; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize];

        for x in 0..X_SIZE {
            for z in 0..Z_SIZE {
                blocks[x as usize][0][z as usize] = 1;
            }
        }

        Self { blocks }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, id: i32) {
        self.blocks[x as usize][y as usize][z as usize] = id;
    }
}

// temporary: for debug rendering
pub struct ChunkPlugin;

impl ChunkPlugin {
    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<Materials>,
    ) {
        let render_distance = 16f32;
        let player_x = 0.0;
        let player_y = 0.0;
        let player_z = 0.0;

        let min_x = ((player_x - render_distance) / (X_SIZE as f32)).floor() as i32 * X_SIZE;
        let max_x = ((player_x + render_distance) / (X_SIZE as f32)).ceil() as i32 * X_SIZE;
        let min_y = ((player_y - render_distance) / (Y_SIZE as f32)).floor() as i32 * Y_SIZE;
        let max_y = ((player_y + render_distance) / (Y_SIZE as f32)).ceil() as i32 * Y_SIZE;
        let min_z = ((player_z - render_distance) / (Z_SIZE as f32)).floor() as i32 * Z_SIZE;
        let max_z = ((player_z + render_distance) / (Z_SIZE as f32)).ceil() as i32 * Z_SIZE;

        for cx in min_x..max_x {
            for cz in min_z..max_z {
                for cy in min_y..max_y {
                    let offset_x = cx * X_SIZE;
                    let offset_y = cy * Y_SIZE;
                    let offset_z = cz * Z_SIZE;
                    let x = (player_x + offset_x as f32) as usize;
                    let y = (player_y + offset_y as f32) as usize;
                    let z = (player_z + offset_z as f32) as usize;
                    
                    let chunk = chunks.get_chunk_at_mut(cx, cy, cz);
                    if x < X_SIZE as usize && y < Y_SIZE as usize && z < Z_SIZE as usize {
                        let block = chunk.blocks[x][y][z];                    
                        let material = materials.get_from_id(&block.try_into().unwrap());
    
                        if material.is_none() {
                            println!("Block not found! {}", block);
                            continue;
                        }
    
                        let material = material.unwrap();
    
                        let scale = 0.2;
                        let x = (x as f32 + cx as f32) * scale;
                        let y = (y as f32 + cy as f32) * scale;
                        let z = (z as f32 + cz as f32) * scale;
    
                        commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(Cube { size: scale })),
                            material: material.bevy_material.clone(),
                            transform: Transform::from_translation(Vec3::new(x, y, z)),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }
}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Chunks::new())
            .add_startup_system(ChunkPlugin::render_blocks);
    }
}
