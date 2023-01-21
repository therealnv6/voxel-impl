use bevy::{
    ecs::schedule::ShouldRun,
    prelude::{
        Assets, Commands, Mesh, PbrBundle, ResMut, StageLabel, StandardMaterial, State,
        SystemLabel, SystemSet, Transform, Vec3, Visibility,
    },
    scene::SceneBundle,
};

use crate::chunk::container::DomainChunk;

use super::container::{self, loaded::LoadedChunks, Chunks};

pub struct ChunkPlugin;
pub struct ChunkStage;

impl ChunkPlugin {
    pub fn render_queue_check() -> ShouldRun {
        return container::get_update_queue().has_queue().into();
    }

    pub fn render_blocks(
        mut commands: Commands,
        mut chunks: ResMut<Chunks>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut bevy_materials: ResMut<Assets<StandardMaterial>>,
        mut state: ResMut<State<ChunkLoadState>>,
        mut loaded_chunks: ResMut<LoadedChunks>,
    ) {
        let mut outer_most_x = 0;

        for (pos, blocks) in container::get_update_queue().pull(0..1) {
            let [x, z] = Chunks::delinearize(pos);
            let chunk = chunks.get_domain_at_mut([x, z]);

            if loaded_chunks.is_chunk_loaded(chunk) && !chunk.dirty {
                continue;
            }

            loaded_chunks.add_rendered_chunk(pos as i32);

            for i in 0..blocks.len() {
                chunk.set_block_domain(i, blocks[i]);
            }

            let mesh = chunk.get_mesh();
            let handle = meshes.add(mesh);

            if chunk.world_pos.x > outer_most_x {
                outer_most_x = chunk.world_pos.x;
            }

            const SCALE: f32 = 1.0;

            if let None = chunk.entity {
                chunk.entity = Some(commands.spawn_empty().id());
            }

            let entity = chunk.entity.unwrap();
            let mut commands = commands.entity(entity);

            commands.remove::<Visibility>().insert(PbrBundle {
                mesh: handle,
                material: bevy_materials.add(StandardMaterial {
                    perceptual_roughness: 0.47,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    chunk.world_pos.x as f32 * SCALE,
                    0.0,
                    chunk.world_pos.y as f32 * SCALE,
                ))
                .with_scale(Vec3::new(SCALE, SCALE, SCALE)),
                ..Default::default()
            });

            chunk.dirty = false;
        }

        if let ChunkLoadState::Render = state.current() {
            state.overwrite_set(ChunkLoadState::Wait).unwrap();
        }

        loaded_chunks.pull_unload().iter().for_each(|chunk| {
            let [x, z] = Chunks::delinearize(*chunk);
            let chunk = chunks.get_domain_at_mut([x, z]);

            if let Some(entity) = chunk.entity {
                let entity = commands.get_entity(entity);

                if let Some(mut entity) = entity {
                    entity
                        .insert(SceneBundle {
                            visibility: Visibility { is_visible: false },
                            ..Default::default()
                        })
                        .remove::<PbrBundle>();
                }
            }
        });
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
        app.insert_resource(Chunks::default())
            .insert_resource(LoadedChunks::default())
            .add_state(ChunkLoadState::Render)
            .add_system_set(
                SystemSet::on_enter(ChunkLoadState::Render)
                    .with_run_criteria(ChunkPlugin::render_queue_check)
                    .with_system(ChunkPlugin::render_blocks),
            );
    }
}
