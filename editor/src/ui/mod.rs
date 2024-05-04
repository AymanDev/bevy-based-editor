pub mod editor_screen_space;
pub mod left_panel;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    core::Name,
    ecs::system::{Commands, ResMut},
    math::primitives::{Cuboid, Plane3d},
    pbr::{AmbientLight, PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{Mesh, Meshable},
    },
    transform::components::Transform,
};
use bevy_mod_picking::PickableBundle;

use self::editor_screen_space::OccupiedScreenSpace;

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.,
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..Default::default()
        })
        .insert(Name::new("Plane"))
        .insert(PickableBundle::default());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Cuboid"))
        .insert(PickableBundle::default());

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 20500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        .insert(Name::new("Point Light"))
        .insert(PickableBundle::default());
}

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<OccupiedScreenSpace>()
            .add_systems(Startup, init)
            .add_systems(Update, left_panel::draw);
    }
}
