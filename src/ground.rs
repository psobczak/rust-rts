use bevy::prelude::*;
use bevy_rts_camera::Ground;
use bevy_xpbd_3d::{components::RigidBody, prelude::Collider};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Collider::cuboid(50.0, 0.0, 50.0),
        RigidBody::Static,
        Name::from("Ground"),
        Ground,
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::WHITE),
            // transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
    ));
}
