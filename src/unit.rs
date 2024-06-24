use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub const UNIT_SIZE: f32 = 0.5;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spaw_units);
    }
}

#[derive(Component)]
pub struct Unit;

fn spaw_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..7 {
        commands.spawn((
            Name::from(i.to_string()),
            Unit,
            PickableBundle::default(),
            PbrBundle {
                material: materials.add(Color::GRAY),
                mesh: meshes.add(Cuboid::from_size(Vec3::splat(UNIT_SIZE))),
                transform: Transform::from_xyz(3.0 + i as f32, UNIT_SIZE / 2.0, 0.0),
                ..Default::default()
            },
        ));
    }
}
