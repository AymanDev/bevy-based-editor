use bevy::{
    asset::{Assets, Handle},
    core::Name,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    hierarchy::{Children, Parent},
    prelude::*,
    render::mesh::Mesh,
    transform::components::GlobalTransform,
};
use bevy_egui::{
    egui::{self, collapsing_header::CollapsingState, RichText, Sense, Ui},
    EguiContext, EguiContexts,
};

use crate::active_selection::{extract_scale_and_transform, ActiveSelection};

type SingleQuerySelector<'a> = (
    Entity,
    Option<&'a Name>,
    Option<&'a GlobalTransform>,
    Option<&'a Handle<Mesh>>,
);

type RootQuerySelector<'a> = (SingleQuerySelector<'a>, Option<&'a Children>);

pub fn draw(
    mut commands: Commands,
    mut contexts: EguiContexts,
    entities: Query<RootQuerySelector, (Without<Parent>, Without<EguiContext>)>,
    entities_with_parent: Query<RootQuerySelector, With<Parent>>,
    mut active_selection: ResMut<ActiveSelection>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ctx = contexts.ctx_mut();
    egui::SidePanel::left("Hierarchy")
        .resizable(true)
        .min_width(200.)
        .show(ctx, |ui| {
            ui.add_space(10.);
            ui.heading(RichText::new("Hierarchy").size(30.));
            ui.add_space(5.);

            draw_tree_root(
                ui,
                &mut commands,
                entities,
                entities_with_parent,
                &mut active_selection,
                &mut meshes,
            );

            ui.allocate_rect(ui.available_rect_before_wrap(), Sense::hover());
        });
}

fn draw_tree_root(
    ui: &mut Ui,
    commands: &mut Commands,
    query: Query<RootQuerySelector, (Without<Parent>, Without<EguiContext>)>,
    query_with_parent: Query<RootQuerySelector, With<Parent>>,
    active_selection: &mut ResMut<ActiveSelection>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    // Walking over all entities for hierarchy
    for (single_components, children) in query.iter() {
        let (entity, name, transform, mesh) = single_components;

        let name = unpack_name(name);

        let is_entity_selected =
            active_selection.has_selection && entity.index() == active_selection.entity_id;

        match children {
            None => {
                // No childrens for this entity, drawing as single
                if draw_single(is_entity_selected, ui, name) {
                    change_selection(commands, entity, transform, mesh, active_selection, meshes);
                }
            }
            Some(children) => {
                // Childrens found for this entity, walking down the tree
                draw_tree_node(
                    ui,
                    commands,
                    single_components,
                    children,
                    &query_with_parent,
                    active_selection,
                    meshes,
                );
            }
        };
    }
}

fn draw_tree_node(
    ui: &mut Ui,
    commands: &mut Commands,
    components: SingleQuerySelector,
    children: &Children,
    query_with_parent: &Query<RootQuerySelector, With<Parent>>,
    active_selection: &mut ResMut<ActiveSelection>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let (entity, name, transform, mesh) = components;

    let name = unpack_name(name);

    let text = RichText::new(name).size(18.);
    let id = ui.make_persistent_id(name);

    let is_entity_selected =
        active_selection.has_selection && entity.index() == active_selection.entity_id;

    CollapsingState::load_with_default_open(ui.ctx(), id, true)
        .show_header(ui, |ui| {
            // Header for tree node
            let mut heading = ui.heading(text);

            if is_entity_selected {
                heading = heading.highlight();
            }

            if heading.clicked() {
                change_selection(commands, entity, transform, mesh, active_selection, meshes);
            }
        })
        .body(|ui| {
            // Walking over all childrens for current entity
            for children_entity in children {
                // Obtainint common components for drawing in hierarchy
                let (components, children) = match query_with_parent.get(*children_entity) {
                    Ok(query) => query,
                    Err(_) => {
                        continue;
                    }
                };

                // This entity has childrens too
                // Walking down the tree
                if let Some(children) = children {
                    draw_tree_node(
                        ui,
                        commands,
                        components,
                        children,
                        query_with_parent,
                        active_selection,
                        meshes,
                    );
                    continue;
                }

                // This entity doesn't have any more childrens
                // Drawing as single

                let (entity, name, transform, mesh) = components;

                let name = unpack_name(name);
                let is_entity_selected =
                    active_selection.has_selection && entity.index() == active_selection.entity_id;

                if draw_single(is_entity_selected, ui, name) {
                    change_selection(commands, entity, transform, mesh, active_selection, meshes);
                }
            }
        });
}

/**
 * Return true if we clicked on it
 */
fn draw_single(is_entity_selected: bool, ui: &mut Ui, name: &str) -> bool {
    let mut heading = ui.heading(name);

    if is_entity_selected {
        heading = heading.highlight();
    }

    heading.clicked()
}

fn change_selection(
    commands: &mut Commands,
    entity: Entity,
    transform: Option<&GlobalTransform>,
    mesh: Option<&Handle<Mesh>>,
    active_selection: &mut ResMut<ActiveSelection>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    active_selection.has_selection = true;
    let (scale, transform) = extract_scale_and_transform(transform, mesh, meshes);

    active_selection.scale = scale;
    active_selection.transform = transform;
    active_selection.entity_id = entity.index();
}

fn unpack_name(name: Option<&Name>) -> &str {
    name.map_or("Unnamed", |name| name.as_str())
}
