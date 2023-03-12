use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

#[derive(Component)]
pub struct Attractor {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Attractor {
    pub fn attractor_system(
        mut query: Query<(&Attractor, &mut Transform), With<Attractor>>,
        time: Res<Time>,
        mut lines: ResMut<DebugLines>,
    ) {
        let dt: f32 = time.delta_seconds();
        for (attractor, mut transform) in &mut query {
            let mut position = transform.translation;
            let dx: f32 = attractor.sigma * (position.y - position.x);
            let dy: f32 = position.x * (attractor.rho - position.z) - position.y;
            let dz: f32 = position.x * position.y - attractor.beta * position.z;

            position += Vec3 {
                x: dx,
                y: dy,
                z: dz,
            } * dt;

            lines.line_colored(transform.translation, position, 30.0, Color::WHITE);

            transform.translation = position;
        }
    }
}

impl Default for Attractor {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}
