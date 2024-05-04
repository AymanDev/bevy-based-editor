mod entity_info;
mod transform_info;

use bevy::ecs::system::ResMut;
use bevy_egui::{
    egui::{self, RichText, Sense},
    EguiContexts,
};

use crate::active_selection::ActiveSelection;

pub fn draw(mut contexts: EguiContexts, active_selection: ResMut<ActiveSelection>) {
    egui::SidePanel::right("Inspector")
        .resizable(true)
        .min_width(300.)
        .show(contexts.ctx_mut(), |ui| {
            ui.layout().horizontal_justify();
            ui.add_space(10.);
            ui.heading(RichText::new("Inspector").size(30.));
            ui.add_space(5.);

            if !active_selection.has_selection {
                ui.allocate_rect(ui.available_rect_before_wrap(), Sense::hover());
                return;
            }

            entity_info::draw(ui, &active_selection);
            transform_info::draw(ui, &active_selection);

            ui.allocate_rect(ui.available_rect_before_wrap(), Sense::hover());
        });
}
