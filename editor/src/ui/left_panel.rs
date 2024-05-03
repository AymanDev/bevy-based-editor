use bevy::ecs::system::ResMut;
use bevy_egui::{egui, EguiContexts};

use super::editor_screen_space::OccupiedScreenSpace;

pub fn draw(mut contexts: EguiContexts, mut occupied_screen_space: ResMut<OccupiedScreenSpace>) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("Hierarchy")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Hierarchy 1");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
