use bevy::{
    prelude::{
        shape::Cube, Assets, Camera, Commands, IVec3, IntoSystemDescriptor, Mesh, PbrBundle,
        Plugin, Query, Res, ResMut, Resource, StageLabel, State, SystemLabel, SystemSet,
        SystemStage, Transform, Vec3, With,
    },
    utils::HashMap,
};

use crate::{
    camera::{self, CameraController},
    debug_camera,
    material::{self, Materials},
    player::PlayerEntity,
};

pub const X_SIZE: i32 = 12;
pub const Y_SIZE: i32 = 12;
pub const Z_SIZE: i32 = 12;

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
        let chunk_x = x / X_SIZE * X_SIZE + X_SIZE / 2;
        let chunk_y = y / Y_SIZE * Y_SIZE + Y_SIZE / 2;
        let chunk_z = z / Z_SIZE * Z_SIZE + Z_SIZE / 2;
        let id = ((chunk_x / X_SIZE) << 20)
            | ((chunk_y / Y_SIZE) << 10)
            | ((chunk_z / Z_SIZE) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get_mut(&id).unwrap();
        }

        let chunk = Chunk::new(chunk_x, chunk_y, chunk_z);

        self.chunks.insert(id, chunk);
        self.get_chunk_at_mut(x, y, z)
    }

    pub fn get_chunk_at(&mut self, x: i32, y: i32, z: i32) -> &Chunk {
        let chunk_x = x / X_SIZE * X_SIZE + X_SIZE / 2;
        let chunk_y = y / Y_SIZE * Y_SIZE + Y_SIZE / 2;
        let chunk_z = z / Z_SIZE * Z_SIZE + Z_SIZE / 2;
        let id = ((chunk_x / X_SIZE) << 20)
            | ((chunk_y / Y_SIZE) << 10)
            | ((chunk_z / Z_SIZE) as i32 & 1023);

        if self.chunks.contains_key(&id) {
            return self.chunks.get(&id).unwrap();
        }

        let chunk = Chunk::new(chunk_x, chunk_y, chunk_z);

        self.chunks.insert(id, chunk);
        self.get_chunk_at(x, y, z)
    }
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
    blocks: [[[i32; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize],
    pub world_pos: IVec3,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut blocks = [[[0; X_SIZE as usize]; Y_SIZE as usize]; Z_SIZE as usize];

        for x in 0..X_SIZE {
            for y in 0..Y_SIZE {
                for z in 0..Z_SIZE {
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

// temporary: for debug rendering
pub struct ChunkPlugin;
pub struct ChunkStage;

impl ChunkPlugin {
    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<Materials>,
        query: Query<(&Transform, &CameraController)>,
        mut state: ResMut<State<ChunkLoadState>>,
    ) {
        let transform = query.get_single();

        if let Ok((transform, _)) = transform {
            let translation = transform.translation;

            let render_distance = 4f32;
            let player_x = translation.x;
            let player_y = translation.y;
            let player_z = translation.z;

            let min_x = ((player_x - render_distance) / (X_SIZE as f32)).floor() as i32 * X_SIZE;
            let max_x = ((player_x + render_distance) / (X_SIZE as f32)).ceil() as i32 * X_SIZE;
            let min_y = ((player_y - render_distance) / (Y_SIZE as f32)).floor() as i32 * Y_SIZE;
            let max_y = ((player_y + render_distance) / (Y_SIZE as f32)).ceil() as i32 * Y_SIZE;
            let min_z = ((player_z - render_distance) / (Z_SIZE as f32)).floor() as i32 * Z_SIZE;
            let max_z = ((player_z + render_distance) / (Z_SIZE as f32)).ceil() as i32 * Z_SIZE;

            for cx in min_x..max_x {
                for cy in min_y..max_y {
                    for cz in min_z..max_z {
                        let offset_x = cx / X_SIZE;
                        let offset_y = cy / Y_SIZE;
                        let offset_z = cz / Z_SIZE;

                        let scaled_x = offset_x as usize;
                        let scaled_y = offset_y as usize;
                        let scaled_z = offset_z as usize;

                        let chunk = chunks.get_chunk_at_mut(cx, cy, cz);

                        if scaled_x < X_SIZE as usize
                            && scaled_y < Y_SIZE as usize
                            && scaled_z < Z_SIZE as usize
                        {
                            let block = chunk.blocks[scaled_x][scaled_y][scaled_z];

                            if block == 0 {
                                continue;
                            }

                            let material = materials.get_from_id(&block.try_into().unwrap());

                            if material.is_none() {
                                println!("Block not found! {}", block);
                                continue;
                            }

                            let material = material.unwrap();

                            let scale = 0.4;
                            let render_x = cx as f32 * scale;
                            let render_y = cy as f32 * scale;
                            let render_z = cz as f32 * scale;

                            commands.spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(Cube { size: scale })),
                                material: material.bevy_material.clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    render_x, render_y, render_z,
                                )),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            if let ChunkLoadState::Render = state.current() {
                state.overwrite_set(ChunkLoadState::Wait).unwrap();
            }
        }
    }
}

#[derive(StageLabel)]
pub struct ChunkLoadingStage;

#[derive(SystemLabel, Debug, Hash, Clone, PartialEq, Eq)]
pub enum ChunkLoadState {
    Wait,
    Render,
}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.update();
        app.insert_resource(Chunks::new())
            .add_state(ChunkLoadState::Render)
            .add_system_set(
                SystemSet::on_enter(ChunkLoadState::Render).with_system(
                    ChunkPlugin::render_blocks
                        .after(material::MaterialPlugin::init_materials)
                        .after(debug_camera)
                        .label(ChunkLoadState::Render),
                ),
            );
    }
}
