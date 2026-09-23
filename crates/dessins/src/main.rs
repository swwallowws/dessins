use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_nannou::prelude::*;
use bevy_nannou::NannouPlugin;
use dessins::{dessin_with_variables::DessinVariant, model::Model};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    };

    let default_plugins = DefaultPlugins.set(window_plugin);

    let model = Model::new(DessinVariant::RoundedSimpleFractal);

    App::new()
        .add_plugins((default_plugins, NannouPlugin, EguiPlugin))
        .insert_resource(model)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_active_dessin, draw_dessin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(render::NannouCamera);
}

fn update_active_dessin(
    mut model: ResMut<Model>,
    time: Res<Time<Virtual>>,
    egui_ctx: EguiContexts,
) {
    let (changed, color) = model.update_active_dessin(egui_ctx, *time);

    if let Some(new_color) = color {
        model.color = new_color;
    }

    if changed || !model.initialized() {
        model.calculate_shapes();
    }
}

fn draw_dessin(draw: Single<&Draw>, model: Res<Model>) {
    // TODO: alpha does not work https://github.com/swwallowws/dessins/issues/61
    draw.background().srgba(0.1, 0.1, 0.1, 0.85);

    model.draw_points(draw);
}
