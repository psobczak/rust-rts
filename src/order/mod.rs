mod move_order;

use bevy::prelude::*;
use move_order::Move;

use crate::{selection::Selected, unit::Unit};

const ORDER_KEYS: [KeyCode; 3] = [KeyCode::KeyM, KeyCode::KeyP, KeyCode::KeyS];

pub struct OrderPlugin;

#[derive(Component, Reflect)]
enum Order {
    Move(Vec3),
    Attack(Entity),
    Patrol(PatrolDetails),
}

#[derive(Reflect)]
struct PatrolDetails {
    entity: Entity,
    original_position: Vec3,
    patrol_target: Vec3,
}

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Order>()
            .register_type::<Move>()
            .add_systems(
                Update,
                (
                    add_move_order,
                    add_patrol_order,
                    execute_order,
                    complete_order,
                ),
            );
    }
}

fn add_patrol_order(
    mut commands: Commands,
    selected_units: Query<(Entity, &GlobalTransform), (With<Unit>, With<Selected>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Right) && keyboard.pressed(KeyCode::KeyP) {
        for (unit, transform) in &selected_units {
            commands.entity(unit).insert(Order::Patrol(PatrolDetails {
                original_position: transform.translation(),
                patrol_target: Vec3::new(-4.0, 0.15, -3.0),
                entity: unit,
            }));
        }
    }
}

pub fn add_move_order(
    mut commands: Commands,
    selected_units: Query<Entity, (With<Unit>, With<Selected>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Right) && !keyboard.any_pressed(ORDER_KEYS) {
        for unit in &selected_units {
            commands
                .entity(unit)
                .insert(Order::Move(Vec3::new(-1.0, 0.15, -3.0)));
        }
    }
}

fn execute_order(mut query: Query<(&mut Transform, &Order)>, time: Res<Time>) {
    for (mut transform, order) in &mut query {
        match order {
            Order::Move(target) => {
                transform.look_at(*target, Vec3::Y);
                let direction = transform.forward();
                transform.translation += direction * time.delta_seconds();
            }
            Order::Attack(_) => todo!(),
            Order::Patrol(patrol_details) => {
                transform.look_at(patrol_details.patrol_target, Vec3::Y);
                let direction = transform.forward();
                transform.translation += direction * time.delta_seconds();
            }
        }
    }
}

fn complete_order(mut commands: Commands, query: Query<(Entity, &Order, &GlobalTransform)>) {
    for (entity, order, transform) in &query {
        match order {
            Order::Move(target) => {
                if transform.translation().abs_diff_eq(*target, 0.5) {
                    commands.entity(entity).remove::<Order>();
                }
            }
            Order::Attack(_) => todo!(),
            Order::Patrol(patrol) => {
                if transform
                    .translation()
                    .abs_diff_eq(patrol.patrol_target, 0.5)
                {
                    commands.entity(entity).insert(Order::Patrol(PatrolDetails {
                        original_position: patrol.patrol_target,
                        patrol_target: patrol.original_position,
                        entity,
                    }));
                }
            }
        }
    }
}
