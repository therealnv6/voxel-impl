use bevy::{
    prelude::{App, Camera3dBundle, Commands, PointLight, PointLightBundle, Transform, Vec3},
    DefaultPlugins,
};
use chunk::ChunkPlugin;
use material::MaterialPlugin;

pub mod chunk;
pub mod material;
pub mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin)
        .add_plugin(ChunkPlugin)
        .add_startup_system(debug_camera)
        .run();
}

pub fn debug_camera(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(17.0, 17.0, 17.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
