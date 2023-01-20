use bevy::prelude::{
    AlphaMode, Assets, Color, Commands, IntoSystemDescriptor, Mesh, PbrBundle, Query, ResMut,
    StageLabel, StandardMaterial, State, SystemLabel, SystemSet, Transform, Vec3,
};
use ndshape::ConstShape;

use crate::{
    camera::CameraController,
    chunk::{container::DomainChunk, ChunkShape, X_SIZE, Z_SIZE},
};

use super::container::{self, Chunks, LoadedChunks};

pub struct ChunkPlugin;
pub struct ChunkStage;

impl ChunkPlugin {
    pub fn render_queue_check(mut state: ResMut<State<ChunkLoadState>>) {
        if let ChunkLoadState::Wait = state.current() {
            if container::get_update_queue().has_queue() {
                state
                    .overwrite_set(ChunkLoadState::Render)
                    .expect("Unable to set state!");
            }
        }
    }

    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut bevy_materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(&mut Transform, &CameraController)>,
        mut state: ResMut<State<ChunkLoadState>>,
        mut loaded_chunks: ResMut<LoadedChunks>,
    ) {
        // let transform = query.get_single_mut();
        // if let Ok((transform, _)) = transform {
        // let translation = transform.translation;
        // let render_distance = 12f32;

        // let player_x = translation.x;
        // let player_z = translation.z;

        // let min_x = ((player_x / X_SIZE as f32) - render_distance) as i32;
        // let max_x = ((player_x / X_SIZE as f32) + render_distance) as i32;
        // let min_z = ((player_z / Z_SIZE as f32) - render_distance) as i32;
        // let max_z = ((player_z / Z_SIZE as f32) + render_distance) as i32;

        let mut outer_most_x = 0;
        let mut loaded = Vec::<i32>::new();

        for linear in container::get_update_queue().pull() {
            let [x, z] = Chunks::delinearize(linear);
            let chunk = chunks.get_domain_at_mut([x, z]);

            loaded.push(linear as i32);

            if loaded_chunks.is_chunk_loaded(chunk) {
                println!("not rendering loaded chunk! {linear}");
                continue;
            }

            let mesh = chunk.get_mesh();
            let handle = meshes.add(mesh);

            if chunk.world_pos.x > outer_most_x {
                outer_most_x = chunk.world_pos.x;
            }

            const SCALE: f32 = 1.0;

            println!("{}, {}", chunk.world_pos.x, chunk.world_pos.y);

            commands.spawn(PbrBundle {
                mesh: handle,
                material: bevy_materials.add(StandardMaterial {
                    perceptual_roughness: 0.47,
                    // alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    chunk.world_pos.x as f32 * SCALE,
                    // chunk.world_pos.y as f32 * SCALE,
                    0.0,
                    chunk.world_pos.y as f32 * SCALE,
                ))
                .with_scale(Vec3::new(SCALE, SCALE, SCALE)),
                ..Default::default()
            });
        }

        if let ChunkLoadState::Render = state.current() {
            state.overwrite_set(ChunkLoadState::Wait).unwrap();
        }

        loaded_chunks.replace(loaded);
        // }
    }
}

#[derive(StageLabel)]
pub struct ChunkLoadingStage;

#[derive(SystemLabel, Debug, Hash, Clone, PartialEq, Eq)]
pub enum ChunkLoadState {
    Wait,
    Render,
}

impl bevy::app::Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.update();
        app.insert_resource(Chunks::new())
            .insert_resource(LoadedChunks::default())
            .add_state(ChunkLoadState::Render)
            .add_system(ChunkPlugin::render_queue_check)
            .add_system_set(
                SystemSet::on_enter(ChunkLoadState::Render)
                    .with_system(ChunkPlugin::render_blocks.label(ChunkLoadState::Render)),
            );
    }
}
