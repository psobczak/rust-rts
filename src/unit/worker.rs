use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_xpbd_3d::{components::RigidBody, prelude::Collider};

use crate::{building::WorkingInMine, order::OrdersQueue};

use super::{Unit, Worker, UNIT_SIZE};

pub fn spawn_workers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..7 {
        commands.spawn((
            Collider::cuboid(UNIT_SIZE, UNIT_SIZE, UNIT_SIZE),
            RigidBody::Dynamic,
            OrdersQueue::default(),
            Name::from(i.to_string()),
            Unit,
            Worker,
            PickableBundle::default(),
            PbrBundle {
                material: materials.add(Color::GRAY),
                mesh: meshes.add(Cuboid::from_size(Vec3::splat(UNIT_SIZE))),
                transform: Transform::from_xyz(3.0 + i as f32, UNIT_SIZE + 0.5, 0.0),
                ..Default::default()
            },
        ));
    }
}

pub fn hide_workers_in_gold_mine(mut workers: Query<&mut Visibility, Added<WorkingInMine>>) {
    for mut visibility in workers.iter_mut() {
        info!("Should hide");
        *visibility = Visibility::Hidden;
    }
}
