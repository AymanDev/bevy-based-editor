use bevy::{
    app::{Plugin, Update},
    asset::{Assets, Handle},
    ecs::{
        event::EventReader,
        system::{Query, ResMut, Resource},
    },
    gizmos::gizmos::Gizmos,
    math::Vec3,
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_mod_picking::{
    events::Pointer,
    selection::{Deselect, Select},
};

#[derive(Resource, Default)]
pub struct ActiveSelection {
    pub entity_id: u32,
    pub transform: Transform,
    pub scale: Vec3,
    pub has_selection: bool,
}

const DEFAULT_SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);

pub fn extract_scale_and_transform(
    transform: Option<&Transform>,
    mesh: Option<&Handle<Mesh>>,
    meshes: &ResMut<Assets<Mesh>>,
) -> (Vec3, Transform) {
    let transform = match transform {
        Some(transform) => *transform,
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

fn select_from_pointer_event(
    mut pointer_select: EventReader<Pointer<Select>>,
    mut active_selection: ResMut<ActiveSelection>,
    query: Query<(Option<&Transform>, Option<&Handle<Mesh>>)>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if pointer_select.is_empty() {
        return;
    }

    for event in pointer_select.read() {
        if let Ok((transform, mesh)) = query.get(event.target) {
            active_selection.has_selection = true;
            active_selection.entity_id = event.target.index();

            let (scale, transform) = extract_scale_and_transform(transform, mesh, &meshes);

            active_selection.transform = transform;
            active_selection.scale = scale;
        }
    }
}

fn deselect_from_pointer_event(
    pointer_deselect: EventReader<Pointer<Deselect>>,
    mut active_selection: ResMut<ActiveSelection>,
) {
    if pointer_deselect.is_empty() {
        return;
    }

    active_selection.has_selection = false;
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
            .add_systems(
                Update,
                (select_from_pointer_event, deselect_from_pointer_event),
            );
    }
}
