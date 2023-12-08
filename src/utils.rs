use bevy::prelude::*;

pub fn grid_update(mut gizmos: Gizmos) {
    const GRID_SIZE: f32 = 10.0;
    const NUM_LINES: f32 = 201.0;

    const HALF_SIZE: f32 = GRID_SIZE * NUM_LINES / 2.0;

    gizmos.line(
        Vec3::new(-HALF_SIZE, 0.0, 0.0),
        Vec3::new(HALF_SIZE, 0.0, 0.0),
        Color::RED,
    ); // X-axis (red)
    gizmos.line(
        Vec3::new(0.0, -HALF_SIZE, 0.0),
        Vec3::new(0.0, HALF_SIZE, 0.0),
        Color::GREEN,
    ); // Y-axis (green)
    gizmos.line(
        Vec3::new(0.0, 0.0, -HALF_SIZE),
        Vec3::new(0.0, 0.0, HALF_SIZE),
        Color::BLUE,
    ); // Z-axis (blue)
}
