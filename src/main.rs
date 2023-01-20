// #![windows_subsystem = "windows"]
use bevy::prelude::PluginGroup;
use bevy::render::settings::{PowerPreference, WgpuSettings};
use bevy::window::{CursorGrabMode, WindowMode};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    pbr::wireframe::WireframePlugin,
    prelude::{
        AmbientLight, App, AssetServer, Camera3dBundle, Color, Commands, Component, Msaa,
        PointLight, PointLightBundle, Query, Res, TextBundle, Transform, Vec3, With,
    },
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, UiRect, Val},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraController;
use chunk::plugin::ChunkPlugin;
use material::MaterialPlugin;

pub mod camera;
pub mod chunk;
pub mod material;
pub mod player;
pub mod terrain;

fn main() {
    App::new()
        .add_startup_system(debug_camera)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                present_mode: PresentMode::Immediate,
                cursor_grab_mode: CursorGrabMode::Confined,
                cursor_visible: false,
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(WgpuSettings {
            power_preference: PowerPreference::HighPerformance,
            constrained_limits: None,
            ..Default::default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MaterialPlugin)
        .add_plugin(ChunkPlugin)
        .add_plugin(WireframePlugin)
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(Msaa { samples: 4 })
        // .add_plugin(BevyVfxBagPlugin)
        // .add_plugin(RaindropsPlugin)
        .add_plugin(AtmospherePlugin)
        .add_system(camera::camera_controller)
        .add_system(camera::chunk_loading)
        .add_system(text_update_system)
        .run();
}

pub fn debug_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            radius: 427.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(233.68, 48.69, 418.04),
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(233.68, 48.69, 418.04))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        CameraController::default(),
        AtmosphereCamera::default(),
        // PostProcessingInput,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::new(
                "fps: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_sections([
            TextSection::new(
                "pos: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Percent(94.0),
                ..Default::default()
            },
            ..Default::default()
        }),
        PosText,
    ));
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct PosText;

#[derive(Component)]
struct CameraIdentifier;

#[cfg(test)]
mod test {
    use crate::chunk::container::Chunks;

    #[test]
    pub fn chunk_test() {
        let mut chunks = Chunks::new();

        let chunk1 = chunks.get_chunk_at(5, 3, 5).world_pos;
        let chunk2 = chunks.get_chunk_at(0, 1, 0).world_pos;
        let chunk3 = chunks.get_chunk_at(3, 5, 2).world_pos;

        assert_eq!(chunk1, chunk2);
        assert_eq!(chunk2, chunk3);
    }

    #[test]
    pub fn chunk_domain_test() {
        let mut chunks = Chunks::new();

        let chunk1 = chunks.get_domain_at(0, 0, 0).world_pos;
        let chunk2 = chunks.get_domain_at(0, 1, 0).world_pos;
        let chunk3 = chunks.get_domain_at(0, 0, 1).world_pos;

        assert_ne!(chunk1, chunk2);
        assert_ne!(chunk2, chunk3);
    }
}
