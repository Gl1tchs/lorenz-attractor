mod attractor;
mod utils;

use attractor::AttractorPlugin;
use utils::grid_update;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(EguiPlugin)
        .add_plugins((LookTransformPlugin, OrbitCameraPlugin::default()))
        .add_plugins(AttractorPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (text_update, grid_update))
        .run();
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_rotate_sensitivity: Vec2::splat(0.08),
                mouse_translate_sensitivity: Vec2::splat(2.5),
                mouse_wheel_zoom_sensitivity: 0.15,
                ..Default::default()
            },
            Vec3::new(-100.0, 100.0, 100.0), // eye
            Vec3::ZERO,                     // target
            Vec3::Y,                        // up
        ));

    {
        const FONT_SIZE: f32 = 20.0;
        let font_asset = asset_server.load("fonts/FiraSans-Bold.ttf");

        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: font_asset.clone(),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: font_asset,
                    font_size: FONT_SIZE,
                    color: Color::GOLD,
                }),
            ])
            // position it to the bottom right corner
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            }),
            FpsText,
        ));
    }

    commands.spawn((TextBundle::from_section(
        "Click the MB2 button and move the mouse to initiate movement.\nHold down the Ctrl key to enable rotation.",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
            font_size: 14.0,
            color: Color::WHITE,
        },
    )
    // position it to the top right corner
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        right: Val::Px(10.0),
        ..default()
    }),));
}

fn text_update(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[1].value = format!("{value:.2}");
        }
    }
}
