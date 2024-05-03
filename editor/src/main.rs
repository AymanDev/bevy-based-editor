mod ui;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use ui::EditorUiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(EditorUiPlugin)
        .add_systems(Update, main_ui)
        .run();
}

fn main_ui(mut contexts: EguiContexts) {
    egui::Window::new("Editor").show(contexts.ctx_mut(), |ui| {
        ui.label("Yoohoo!");
    });
}
