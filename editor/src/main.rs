mod active_selection;
mod camera;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::{
    backends::{egui::EguiBackendSettings, raycast::RaycastBackendSettings},
    DefaultPickingPlugins,
};
use ui::EditorUiPlugin;

use bevy_mod_picking::backends::raycast::bevy_mod_raycast::immediate::RaycastVisibility;

use crate::{active_selection::ActiveSelectionPlugin, camera::EditorCameraPlugin};

fn init(mut config_store: ResMut<GizmoConfigStore>) {
    // let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    for (_, config, _) in config_store.iter_mut() {
        config.depth_bias = -1.
    }
}

fn main() {
    println!("Editor starting...");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(EditorCameraPlugin)
        .add_plugins(EguiPlugin)
        .insert_resource(EguiBackendSettings {
            allow_deselect: true,
        })
        .insert_resource(RaycastBackendSettings {
            raycast_visibility: RaycastVisibility::Ignore,
            ..Default::default()
        })
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(ActiveSelectionPlugin)
        .add_plugins(EditorUiPlugin)
        .run();
}
