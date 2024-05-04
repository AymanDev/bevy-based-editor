mod active_selection;
mod camera;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use ui::EditorUiPlugin;

use crate::{active_selection::ActiveSelectionPlugin, camera::EditorCameraPlugin};

fn init(mut config_store: ResMut<GizmoConfigStore>) {
    for (_, config, _) in config_store.iter_mut() {
        config.depth_bias = -1.
    }
}

fn main() {
    println!("Editor starting...");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Editor".to_owned(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 1280.,
                    min_height: 720.,
                    ..Default::default()
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .add_plugins(EditorCameraPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ActiveSelectionPlugin)
        .add_plugins(EditorUiPlugin)
        .run();
}
