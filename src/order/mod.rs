use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    building::WorkingInMine,
    cursor::MousePosition,
    selection::Selected,
    unit::{Unit, UNIT_SIZE},
};

const ORDER_KEYS: [KeyCode; 3] = [KeyCode::KeyM, KeyCode::KeyP, KeyCode::KeyS];

pub struct OrderPlugin;

#[derive(Component, Reflect, Debug)]
pub enum Order {
    Move(Vec3),
    Attack(Entity),
    Patrol(PatrolDetails),
    Work(Vec3),
}

#[derive(Component, Debug, Reflect, Default)]
pub struct OrdersQueue(VecDeque<Order>);

#[derive(Reflect, Debug)]
pub struct PatrolDetails {
    pub entity: Entity,
    pub original_position: Vec3,
    pub patrol_target: Vec3,
}

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Order>()
            .register_type::<OrdersQueue>()
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

#[allow(clippy::type_complexity)]
fn add_patrol_order(
    mut commands: Commands,
    selected_units: Query<(Entity, &GlobalTransform), (With<Unit>, With<Selected>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
) {
    if mouse.just_pressed(MouseButton::Right) && keyboard.pressed(KeyCode::KeyP) {
        for (unit, transform) in &selected_units {
            let mut patrol_target = mouse_position.world;
            patrol_target.y = UNIT_SIZE / 2.0;
            commands.entity(unit).insert(Order::Patrol(PatrolDetails {
                original_position: transform.translation(),
                patrol_target,
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
    mouse_position: Res<MousePosition>,
) {
    if mouse.just_pressed(MouseButton::Right) && !keyboard.any_pressed(ORDER_KEYS) {
        for unit in &selected_units {
            commands
                .entity(unit)
                .insert(Order::Move(mouse_position.world));
        }
    }
}

fn execute_order(mut query: Query<(&mut Transform, &Order)>, time: Res<Time>) {
    for (mut transform, order) in &mut query {
        match order {
            Order::Move(target) | Order::Work(target) => {
                transform.look_at(*target, Vec3::Y);
                let direction = transform.forward();
                let direction = Vec3::new(direction.x, UNIT_SIZE / 2.0, direction.z);
                info!("Direction: {:?}", direction);
                transform.translation += direction * time.delta_seconds();
            }
            Order::Patrol(patrol_details) => {
                transform.look_at(patrol_details.patrol_target, Vec3::Y);
                let direction = transform.forward();
                transform.translation += direction * time.delta_seconds();
            }
            _ => {}
        }
    }
}

fn complete_order(mut commands: Commands, query: Query<(Entity, &Order, &GlobalTransform)>) {
    for (entity, order, transform) in &query {
        match order {
            Order::Move(target) => {
                if transform.translation().abs_diff_eq(*target, 0.1) {
                    commands.entity(entity).remove::<Order>();
                }
            }
            Order::Work(target) => {
                if transform.translation().abs_diff_eq(*target, 0.1) {
                    commands.entity(entity).remove::<Order>();
                    commands.entity(entity).insert(WorkingInMine);
                }
            }
            Order::Patrol(patrol) => {
                if transform
                    .translation()
                    .abs_diff_eq(patrol.patrol_target, 0.1)
                {
                    commands.entity(entity).insert(Order::Patrol(PatrolDetails {
                        original_position: patrol.patrol_target,
                        patrol_target: patrol.original_position,
                        entity,
                    }));
                }
            }
            _ => {}
        }
    }
}
