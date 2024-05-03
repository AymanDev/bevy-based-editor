mod editor_screen_space;
mod left_panel;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{
        primitives::{Cuboid, Plane3d},
        Vec3,
    },
    pbr::{AmbientLight, PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{Deref, DerefMut},
    render::{
        camera::Projection,
        color::Color,
        mesh::{Mesh, Meshable},
    },
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

use self::editor_screen_space::OccupiedScreenSpace;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.,
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default().mesh()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        ..Default::default()
    });

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..Default::default()
    });
}

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<OccupiedScreenSpace>()
            .add_systems(Startup, init)
            .add_systems(Update, update_camera_transform_system)
            .add_systems(Update, left_panel::draw);
    }
}
