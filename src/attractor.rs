use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct AttractorPlugin;

impl Plugin for AttractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, attractor_setup)
            .add_systems(Update, (attractor_update, attractor_gizmos, attractor_ui));
    }
}

#[derive(Component)]
struct Translation(Vec3);

#[derive(Component)]
struct TranslationHistory {
    positions: Vec<Vec3>,
}

#[derive(Component, Clone, Copy)]
struct Attractor {
    sigma: f32,
    rho: f32,
    beta: f32,
}

#[derive(Component)]
struct AttractorPlaceholder(Attractor);

fn attractor_setup(mut commands: Commands) {
    let attractor = Attractor {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };

    commands.spawn((
        attractor.clone(),
        Translation(Vec3::ONE),
        TranslationHistory {
            positions: Vec::new(),
        },
        AttractorPlaceholder(attractor),
    ));
}

fn attractor_update(
    mut query: Query<(&Attractor, &mut Translation, &mut TranslationHistory)>,
    time: Res<Time>,
) {
    let (attractor, mut translation, mut history) = query.single_mut();

    let dt: f32 = time.delta_seconds();
    let mut position = translation.0;
    let dx: f32 = attractor.sigma * (position.y - position.x);
    let dy: f32 = position.x * (attractor.rho - position.z) - position.y;
    let dz: f32 = position.x * position.y - attractor.beta * position.z;

    position += Vec3 {
        x: dx,
        y: dy,
        z: dz,
    } * dt;

    history.positions.push(position);

    translation.0 = position;
}

fn attractor_gizmos(mut gizmos: Gizmos, query: Query<&TranslationHistory>) {
    let history = query.single();

    gizmos.linestrip(history.positions.clone(), Color::WHITE);
}

fn attractor_ui(
    mut contexts: EguiContexts,
    mut query: Query<(
        &mut Attractor,
        &mut Translation,
        &mut TranslationHistory,
        &mut AttractorPlaceholder,
    )>,
) {
    egui::Window::new("Options")
        .default_width(100.0)
        .collapsible(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let (mut attractor, mut translation, mut history, mut attractor_placeholder) =
                query.single_mut();

            // Use the placeholder to update the sliders without affecting real values
            ui.add(egui::Slider::new(&mut attractor_placeholder.0.sigma, 0.0..=30.0).text("Sigma"));
            ui.add(egui::Slider::new(&mut attractor_placeholder.0.rho, 0.0..=30.0).text("Rho"));
            ui.add(egui::Slider::new(&mut attractor_placeholder.0.beta, 0.0..=30.0).text("Beta"));

            ui.separator();

            if ui
                .add_sized(
                    egui::Vec2::new(ui.available_width(), 20.0),
                    egui::Button::new("Restart"),
                )
                .clicked()
            {
                attractor.sigma = attractor_placeholder.0.sigma;
                attractor.rho = attractor_placeholder.0.rho;
                attractor.beta = attractor_placeholder.0.beta;

                translation.0 = Vec3::ONE;
                history.positions.clear();
            }
        });
}
