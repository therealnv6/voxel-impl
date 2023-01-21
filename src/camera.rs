use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::WireframeConfig,
    prelude::{
        Camera, Component, EulerRot, EventReader, Input, KeyCode, MouseButton, Quat, Query, Res,
        ResMut, StageLabel, Transform, Vec2, Vec3, With,
    },
    text::Text,
    time::Time,
};
use ndshape::ConstShape;
use rand::Rng;

use crate::{
    chunk::{
        container::{self, loaded::LoadedChunks, Chunks, DomainChunk},
        ChunkShape, NoiseShape, X_SIZE, X_SIZE_U32, Y_SIZE_U32, Z_SIZE, Z_SIZE_U32,
    },
    terrain::{self, noise::NoiseData, DebugTerrainGenerator},
    PosText,
};

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub last_chunk_pos: Option<(i32, i32)>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 60.0,
            run_speed: 120.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            last_chunk_pos: None,
        }
    }
}

#[derive(StageLabel)]
pub struct CameraStage;

pub fn camera_controller(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut text: Query<&mut Text, With<PosText>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;

        for (key, axis) in [
            (options.key_forward, &mut axis_input.z),
            (options.key_right, &mut axis_input.x),
            (options.key_up, &mut axis_input.y),
        ] {
            if key_input.pressed(key) {
                *axis += 1.0;
            }
        }

        for (key, axis) in [
            (options.key_back, &mut axis_input.z),
            (options.key_left, &mut axis_input.x),
            (options.key_down, &mut axis_input.y),
        ] {
            if key_input.pressed(key) {
                *axis -= 1.0;
            }
        }

        if key_input.just_pressed(KeyCode::Grave) {
            wireframe_config.global = !wireframe_config.global;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        for mut text in &mut text {
            let x = transform.translation.x;
            let y = transform.translation.y;
            let z = transform.translation.z;

            text.sections[1].value = format!("{x:.2}, {y:.2}, {z:.2}");
        }
    }
}

pub fn update_mouse(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();
    let (mut transform, mut options) = query.single_mut();

    let mut mouse_delta = Vec2::ZERO;

    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    if mouse_delta != Vec2::ZERO {
        options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
            .clamp(-PI / 2., PI / 2.);
        options.yaw -= mouse_delta.x * options.sensitivity * dt;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
    }
}

pub fn reset_chunks(
    mut chunks: ResMut<Chunks>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.pressed(KeyCode::R) {
        loaded_chunks.reset();
        chunks.reset();
    }
}

pub fn chunk_loading(
    mut chunks: ResMut<Chunks>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    noise_data: Res<NoiseData>,
) {
    let (mut transform, mut camera) = query.single_mut();
    let transform = transform.as_mut();
    let camera = camera.as_mut();
    let render_distance = 8f32;

    let translation = transform.translation;
    let (x, z) = (translation.x as i32, translation.z as i32);

    let current = chunks.get_chunk_at([x, z]);
    let world_pos = current.world_pos.clone();

    if camera.last_chunk_pos.is_none() {
        camera.last_chunk_pos = Some((x, z));
    }

    let last_pos = camera.last_chunk_pos.unwrap();
    let previous = chunks.get_chunk_at([last_pos.0, last_pos.1]);

    if world_pos.x != previous.world_pos.x || world_pos.y != previous.world_pos.y {
        let min_x = ((x / X_SIZE as i32) as f32 - render_distance) as i32;
        let max_x = ((x / X_SIZE as i32) as f32 + render_distance) as i32;
        let min_z = ((z / Z_SIZE as i32) as f32 - render_distance) as i32;
        let max_z = ((z / Z_SIZE as i32) as f32 + render_distance) as i32;

        let noise_data = noise_data.as_ref().clone();

        for chunk in loaded_chunks.pull_loaded() {
            let [x, z] = Chunks::delinearize_domain(chunk);

            if (x < min_x || x > max_x) || (z < min_z || z > max_z) {
                loaded_chunks.queue_unload(chunk);
            }
        }

        std::thread::spawn(move || {
            for x in min_x..max_x {
                for z in min_z..max_z {
                    let mut blocks = [0u8; ChunkShape::SIZE as usize];
                    let linear = Chunks::linearize_domain([x, z]);

                    let terrain = terrain::noise::generate_terrain_3d::<ChunkShape, NoiseShape>(
                        &noise_data,
                        rand::thread_rng().gen_range(0..=5000),
                        DebugTerrainGenerator,
                    );

                    assert_eq!(terrain.len(), ChunkShape::SIZE as usize);

                    for i in 0..ChunkShape::SIZE {
                        let [inner_x, inner_y, inner_z] = ChunkShape::delinearize(i as u32);
                        let linearized = ChunkShape::linearize([inner_x, inner_z, inner_y]);

                        if (inner_x > 0 && inner_x < X_SIZE_U32)
                            && (inner_y > 0 && inner_y < Y_SIZE_U32)
                            && (inner_z > 0 && inner_z < Z_SIZE_U32)
                        {
                            blocks[linearized as usize] = terrain[i as usize];
                        }
                    }

                    container::get_update_queue().queue((linear, blocks));
                }
            }
        });

        camera.last_chunk_pos = Some((x, z));
    }
}
