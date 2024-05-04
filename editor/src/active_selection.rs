use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    ecs::{
        event::EventReader,
        system::{Query, Res, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    input::{mouse::MouseButton, ButtonInput},
    math::Vec3,
    render::{color::Color, mesh::Mesh},
    transform::components::{GlobalTransform, Transform},
};
use bevy_egui::EguiContexts;
use bevy_mod_picking::events::{Click, Pointer};

#[derive(Resource, Default)]
pub struct ActiveSelection {
    pub entity_id: u32,
    pub transform: Transform,
    pub scale: Vec3,
    pub has_selection: bool,
}

const DEFAULT_SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);

pub fn extract_scale_and_transform(
    transform: Option<&GlobalTransform>,
    mesh: Option<&Handle<Mesh>>,
    meshes: &ResMut<Assets<Mesh>>,
) -> (Vec3, Transform) {
    let transform = match transform {
        Some(transform) => Transform::from(*transform),
        None => Transform::from_xyz(0., 0., 0.),
    };

    let scale = match mesh {
        Some(mesh) => meshes.get(mesh).map_or(DEFAULT_SCALE, |mesh| {
            mesh.compute_aabb().unwrap_or_default().half_extents.into()
        }),
        None => DEFAULT_SCALE,
    };

    (scale, transform)
}

fn try_to_select_target(
    mut contexts: EguiContexts,
    mut pointer_click: EventReader<Pointer<Click>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut active_selection: ResMut<ActiveSelection>,
    query: Query<(Option<&GlobalTransform>, Option<&Handle<Mesh>>)>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let pointer_over_egui = contexts.ctx_mut().is_pointer_over_area();

    if pointer_over_egui {
        return;
    }

    if pointer_click.is_empty() {
        if input_mouse.pressed(MouseButton::Left) && !pointer_over_egui {
            active_selection.has_selection = false;
        }

        return;
    }

    for event in pointer_click.read() {
        if let Ok((transform, mesh)) = query.get(event.target) {
            active_selection.has_selection = true;
            active_selection.entity_id = event.target.index();

            let (scale, transform) = extract_scale_and_transform(transform, mesh, &meshes);

            active_selection.transform = transform;
            active_selection.scale = scale;
        }
    }
}

fn draw_selection(active_selection: ResMut<ActiveSelection>, mut gizmos: Gizmos) {
    if !active_selection.has_selection {
        return;
    }

    let position = active_selection.transform.translation;

    let scale = active_selection.scale * 2.;

    let transform = Transform::from_xyz(position.x, position.y, position.z).with_scale(scale);

    gizmos.cuboid(transform, Color::RED);

    gizmos.ray(transform.translation, transform.up().into(), Color::GREEN);
    gizmos.ray(
        transform.translation,
        transform.forward().into(),
        Color::BLUE,
    );
    gizmos.ray(transform.translation, transform.right().into(), Color::RED);
}

pub struct ActiveSelectionPlugin;

impl Plugin for ActiveSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ActiveSelection>()
            .add_systems(Update, draw_selection)
            .add_systems(Update, (try_to_select_target,));
    }
}
