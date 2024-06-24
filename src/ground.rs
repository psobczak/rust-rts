use bevy::prelude::*;
use bevy_rts_camera::Ground;

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
        Name::from("Ground"),
        Ground,
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));
}
