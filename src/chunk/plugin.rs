use bevy::prelude::{
    Assets, Commands, IntoSystemDescriptor, Mesh, PbrBundle, Query, ResMut, StageLabel,
    StandardMaterial, State, SystemLabel, SystemSet, Transform, Vec3,
};

use crate::{
    camera::CameraController,
    chunk::{X_SIZE, Z_SIZE},
};

use super::container::Chunks;

pub struct ChunkPlugin;
pub struct ChunkStage;

impl ChunkPlugin {
    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut bevy_materials: ResMut<Assets<StandardMaterial>>,
        mut query: Query<(&mut Transform, &CameraController)>,
        mut state: ResMut<State<ChunkLoadState>>,
    ) {
        let transform = query.get_single_mut();

        if let Ok((mut transform, _)) = transform {
            let translation = transform.translation;
            transform.translation.y += 100.0;
            transform.translation.x += 60.0;

            let render_distance = 6f32;

            let player_x = translation.x;
            let player_z = translation.z;

            let min_x = ((player_x / X_SIZE as f32) - render_distance) as i32;
            let max_x = ((player_x / X_SIZE as f32) + render_distance) as i32;
            let min_z = ((player_z / Z_SIZE as f32) - render_distance) as i32;
            let max_z = ((player_z / Z_SIZE as f32) + render_distance) as i32;

            let mut outer_most_x = 0;

            for chunk_x in min_x..max_x {
                for chunk_z in min_z..max_z {
                    let chunk = chunks.get_domain_at_mut(chunk_x, 0, chunk_z);
                    let mesh = chunk.mesh();

                    let handle = meshes.add(mesh);

                    if chunk.world_pos.x > outer_most_x {
                        outer_most_x = chunk.world_pos.x;
                    }

                    commands.spawn(PbrBundle {
                        mesh: handle,
                        material: bevy_materials.add(StandardMaterial {
                            perceptual_roughness: 0.9,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(Vec3::new(
                            chunk.world_pos.x as f32,
                            chunk.world_pos.y as f32,
                            chunk.world_pos.z as f32,
                        )),
                        ..Default::default()
                    });
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

impl bevy::app::Plugin for ChunkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.update();
        app.insert_resource(Chunks::new())
            .add_state(ChunkLoadState::Render)
            .add_system_set(
                SystemSet::on_enter(ChunkLoadState::Render)
                    .with_system(ChunkPlugin::render_blocks.label(ChunkLoadState::Render)),
            );
    }
}
