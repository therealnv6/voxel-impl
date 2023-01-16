use bevy::{
    prelude::{
        AmbientLight, App, Camera3dBundle, Color, Commands, Component, PointLight,
        PointLightBundle, Transform, Vec3, ResMut, ClearColor,
    },
    DefaultPlugins,
};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_vfx_bag::{BevyVfxBagPlugin, PostProcessingInput, image::raindrops::{Raindrops, RaindropsPlugin}};
use camera::CameraController;
use chunk::ChunkPlugin;
use material::MaterialPlugin;

use crate::player::PlayerEntity;

pub mod camera;
pub mod chunk;
pub mod material;
pub mod player;
pub mod world;

fn main() {
    App::new()
        .add_startup_system(debug_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(BevyVfxBagPlugin)
        .add_plugin(RaindropsPlugin)
        .add_plugin(AtmospherePlugin)
        .add_system(camera::camera_controller)
        .run();
}

pub fn debug_camera(mut commands: Commands) {
    println!("debug camera");
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(17.0, 17.0, 17.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            CameraController::default(),
            AtmosphereCamera::default(),
            PostProcessingInput,
        ))
        .id();
    commands.insert_resource(PlayerEntity { entity: camera });
}

#[derive(Component)]
struct CameraIdentifier;

#[cfg(test)]
mod test {
    use crate::chunk::Chunks;

    #[test]
    pub fn chunk_test() {
        let mut chunks = Chunks::new();

        let chunk1 = chunks.get_chunk_at(5, 3, 5).world_pos;
        let chunk2 = chunks.get_chunk_at(0, 1, 0).world_pos;
        let chunk3 = chunks.get_chunk_at(3, 5, 2).world_pos;

        assert_eq!(chunk1, chunk2);
        assert_eq!(chunk2, chunk3);
    }
}
