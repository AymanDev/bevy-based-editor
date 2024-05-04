use bevy::ecs::system::ResMut;
use bevy_egui::egui::{CollapsingHeader, RichText, Ui};

use crate::active_selection::ActiveSelection;

pub fn draw(ui: &mut Ui, active_selection: &ResMut<ActiveSelection>) {
    CollapsingHeader::new(RichText::new("Entity Info").size(16.))
        .default_open(true)
        .show(ui, |ui| {
            ui.add_space(10.);

            ui.columns(2, |columns| {
                columns[0].label(RichText::new("ID:").size(14.));
                columns[1].label(RichText::new(active_selection.entity_id.to_string()).size(14.));
            });
        });
}
