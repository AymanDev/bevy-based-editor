use bevy::{ecs::system::ResMut, math::EulerRot};
use bevy_egui::egui::{CollapsingHeader, RichText, Ui, WidgetText};

use crate::active_selection::ActiveSelection;

pub fn draw(ui: &mut Ui, active_selection: &ResMut<ActiveSelection>) {
    ui.add_space(10.);

    CollapsingHeader::new(RichText::new("Local Transform Info").size(16.))
        .default_open(true)
        .show(ui, |ui| {
            ui.add_space(10.);

            // Position
            let position = active_selection.transform.translation;
            draw_collapasable_with_xyz_columns(
                ui,
                RichText::new("Position Info").size(16.),
                position.x,
                position.y,
                position.z,
            );

            // Rotation
            let rotation = active_selection
                .transform
                .rotation
                .to_euler(EulerRot::default());
            draw_collapasable_with_xyz_columns(
                ui,
                RichText::new("Rotation Info").size(16.),
                rotation.0,
                rotation.1,
                rotation.2,
            );

            //Scale
            let scale = active_selection.transform.scale;
            draw_collapasable_with_xyz_columns(
                ui,
                RichText::new("Scale Info").size(16.),
                scale.x,
                scale.y,
                scale.z,
            );
        });
}

fn draw_collapasable_with_xyz_columns(
    ui: &mut Ui,
    header: impl Into<WidgetText>,
    x: f32,
    y: f32,
    z: f32,
) {
    ui.add_space(10.);

    CollapsingHeader::new(header)
        .default_open(true)
        .show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].label(RichText::new("X:").size(14.));
                columns[1].label(RichText::new(x.to_string()).size(14.));
            });

            ui.columns(2, |columns| {
                columns[0].label(RichText::new("Y:").size(14.));
                columns[1].label(RichText::new(y.to_string()).size(14.));
            });

            ui.columns(2, |columns| {
                columns[0].label(RichText::new("Z:").size(14.));
                columns[1].label(RichText::new(z.to_string()).size(14.));
            });
        });
}
