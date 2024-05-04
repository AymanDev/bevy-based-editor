use bevy::{
    asset::{Assets, Handle},
    core::Name,
    ecs::{
        entity::Entity,
        system::{Query, ResMut, SystemState},
        world::World,
    },
    math::Vec2,
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy_egui::{
    egui::{self, RichText, Sense},
    EguiContexts,
};

use crate::active_selection::{extract_scale_and_transform, ActiveSelection};

use super::editor_screen_space::OccupiedScreenSpace;

pub fn draw(world: &mut World) {
    let mut system_state: SystemState<(
        EguiContexts,
        ResMut<OccupiedScreenSpace>,
        // There we are selecting everything we need for active selection resource
        Query<(
            Entity,
            Option<&Name>,
            Option<&Transform>,
            Option<&Handle<Mesh>>,
        )>,
        ResMut<ActiveSelection>,
        ResMut<Assets<Mesh>>,
    )> = SystemState::new(world);

    let (mut contexts, mut occupied_screen_space, entities, mut active_selection, meshes) =
        system_state.get_mut(world);

    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("Hierarchy")
        .resizable(true)
        .min_width(200.)
        .show(ctx, |ui| {
            ui.add_space(10.);
            ui.heading(RichText::new("Hierarchy").size(30.));
            ui.add_space(5.);

            ui.layout().vertical_justify();

            // ui.style_mut().spacing.button_padding = egui::Vec2::new(10., 10.);

            for (entity, name, transform, mesh) in entities.iter() {
                let name = name.map_or("Unnamed", |name| name.as_str());

                let is_entity_selected =
                    active_selection.has_selection && entity.index() == active_selection.entity_id;

                let mut text = RichText::new(format!("{}_{}", name, entity.index())).size(16.);

                if is_entity_selected {
                    text = text.color(egui::Color32::BLACK);
                }

                let mut button = egui::Button::new(text);

                if is_entity_selected {
                    button = button.fill(egui::Color32::WHITE);
                }

                let button = ui.add_sized(egui::Vec2::new(ui.available_width(), 30.), button);

                if button.clicked() {
                    active_selection.has_selection = true;

                    let (scale, transform) = extract_scale_and_transform(transform, mesh, &meshes);

                    active_selection.scale = scale;
                    active_selection.transform = transform;

                    active_selection.entity_id = entity.index()
                }

                ui.add_space(5.);
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), Sense::hover());
        })
        .response
        .rect
        .width();
}
