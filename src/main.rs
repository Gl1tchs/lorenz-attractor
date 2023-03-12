mod attractor;

use attractor::Attractor;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{mouse::MouseMotion, Input},
    prelude::*,
    window::{WindowLevel, WindowResolution},
};
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                title: String::from("Lorenz Attractor"),
                decorations: true,
                transparent: false,
                resizable: false,
                window_level: WindowLevel::Normal,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup)
        .add_system(Attractor::attractor_system)
        .add_system(camera_movement_system)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Attractor::default(),
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            transform: Transform::from_xyz(1., 1., 1.).with_scale(Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            }),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 250.0).looking_at(Vec3::ONE, Vec3::Y),
        ..default()
    },));
}

fn camera_movement_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mouse_input: ResMut<Input<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // Camera movement speed
    let move_speed = 10.0;

    // Camera rotation speed
    let look_speed = 0.0025;

    for mut transform in query.iter_mut() {
        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();
        let target = Vec3::new(1.0, 1.0, 1.0);

        // Move the camera based on WASD input
        let mut translation = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            translation += forward;
        }
        if input.pressed(KeyCode::S) {
            translation -= forward;
        }
        if input.pressed(KeyCode::A) {
            translation -= right;
        }
        if input.pressed(KeyCode::D) {
            translation += right;
        }
        if input.pressed(KeyCode::Space) {
            translation += up;
        }
        if input.pressed(KeyCode::LShift) {
            translation -= up;
        }
        translation *= move_speed * time.delta_seconds();
        transform.translation += translation;

        // Rotate the camera based on mouse input
        let mut rotation = Quat::default();
        for event in mouse_motion_events.iter() {
            if mouse_input.pressed(MouseButton::Right) {
                let delta = event.delta;

                // Calculate the orbit rotation quaternion
                let orbit_rotation = Quat::from_axis_angle(right, delta.y * look_speed)
                    * Quat::from_axis_angle(up, -delta.x * look_speed);

                // Apply the orbit rotation to the camera
                let position = transform.translation - target;
                let new_position = orbit_rotation * position;
                transform.translation = target + new_position;

                // Apply the rotation to the camera
                rotation *= orbit_rotation;
            }
        }

        // Apply the rotation to the camera
        transform.rotate(rotation);
    }
}
