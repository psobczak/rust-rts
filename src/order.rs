use bevy::prelude::*;

use crate::{selection::Selected, unit::Unit};

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
        app
        .register_type::<Order>()
        .add_systems(
            Update,
            (
                // add_move_order,
                add_patrol_order,
                execute_order,
                complete_order,
            ),
        );
    }
}

fn add_move_order(
    mut commands: Commands,
    selected_units: Query<Entity, (With<Unit>, With<Selected>)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        for unit in &selected_units {
            commands
                .entity(unit)
                .insert(Order::Move(Vec3::new(5.0, 0.0, -3.0)));
        }
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
                patrol_target: Vec3::new(-1.0, 0.15, -3.0),
                entity: unit
            }));
        }
    }
}

fn execute_order( mut query: Query<(&mut Transform, &Order)>, time: Res<Time>) {
    for (mut transform, order) in &mut query {
        match order {
            Order::Move(target) => {
                transform.look_at(*target, Vec3::Y);
                let direction = transform.forward();
                transform.translation += direction * time.delta_seconds();
            }
            Order::Attack(_) => todo!(),
            Order::Patrol(patrol_details) => {
                let dir = if transform
                    .translation
                    .abs_diff_eq(patrol_details.patrol_target, 0.1)
                {
                    info!("original");
                    patrol_details.original_position
                } else {
                   info!("patrol");

                   patrol_details.patrol_target

                };
                
                transform.look_at(dir, Vec3::Y);
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
            Order::Patrol(_) => {}
        }
    }
}
