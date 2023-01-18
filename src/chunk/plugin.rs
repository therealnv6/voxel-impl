use bevy::prelude::{
    Assets, Color, Commands, IntoSystemDescriptor, Mesh, PbrBundle, Query, Res, ResMut, StageLabel,
    StandardMaterial, State, SystemLabel, SystemSet, Transform, Vec3,
};

use crate::{
    camera::CameraController,
    chunk::{X_SIZE, Y_SIZE, Z_SIZE},
    material::{Materials},
};

use super::container::Chunks;

pub struct ChunkPlugin;
pub struct ChunkStage;

impl ChunkPlugin {
    pub fn render_chunk_debug(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut bevy_materials: ResMut<Assets<StandardMaterial>>,
        mut state: ResMut<State<ChunkLoadState>>,
    ) {
        for i in 0..16 {
            let chunk = chunks.get_domain_at(i, 0, 0);
            let mesh = meshes.add(chunk.mesh());

            commands.spawn(PbrBundle {
                mesh,
                // material: material.bevy_material.clone(),
                material: bevy_materials.add(StandardMaterial {
                    base_color: Color::rgb(0.471, 0.513, 0.21),
                    perceptual_roughness: 0.9,
                    unlit: true,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    chunk.world_pos.x as f32,
                    chunk.world_pos.y as f32,
                    chunk.world_pos.z as f32,
                )),
                ..Default::default()
            });

            if let ChunkLoadState::Render = state.current() {
                state.overwrite_set(ChunkLoadState::Wait).unwrap();
            }
        }
    }

    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        _materials: Res<Materials>,
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
            let player_y = translation.y;
            let player_z = translation.z;

            let min_x = ((player_x / X_SIZE as f32) - render_distance) as i32;
            let max_x = ((player_x / X_SIZE as f32) + render_distance) as i32;
            let min_y = ((player_y / Y_SIZE as f32) - render_distance / 2.0) as i32;
            let max_y = ((player_y / Y_SIZE as f32) + render_distance / 2.0) as i32;
            let min_z = ((player_z / Z_SIZE as f32) - render_distance) as i32;
            let max_z = ((player_z / Z_SIZE as f32) + render_distance) as i32;

            let mut outer_most_x = 0;

            for chunk_x in min_x..max_x {
                for chunk_y in min_y..max_y {
                    for chunk_z in min_z..max_z {
                        let chunk = chunks.get_domain_at_mut(chunk_x, chunk_y, chunk_z);
                        let mesh = chunk.mesh();

                        let handle = meshes.add(mesh);

                        if chunk.world_pos.x > outer_most_x {
                            outer_most_x = chunk.world_pos.x;
                        }

                        commands.spawn(PbrBundle {
                            mesh: handle,
                            // material: material.bevy_material.clone(),
                            material: bevy_materials.add(StandardMaterial {
                                base_color: Color::rgb(0.471, 0.513, 0.21),
                                perceptual_roughness: 0.9,
                                unlit: true,
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
            }

            println!("{outer_most_x}");

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