use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};

use crate::{order::Order, selection::Selected};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ui_example_system, draw_patrol_path, draw_bottom_panel),
        );
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    selected_units: Query<(&Name, Option<&Order>, &GlobalTransform), With<Selected>>,
) {
    egui::Window::new("Selected Units")
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::ZERO)
        .collapsible(false)
        .show(contexts.ctx_mut(), |ui| {
            for (name, order, transform) in &selected_units {
                ui.label(name.to_string());
                ui.label(format!("Current position: {}", transform.translation()));
                if let Some(order) = order {
                    match order {
                        Order::Move(target) => {
                            ui.label(format!("Moving to {:?}", target));
                        }
                        Order::Attack(_) => todo!(),
                        Order::Patrol(patrol) => {
                            ui.label(format!(
                                "Patrolling from {:?} to {:?}",
                                patrol.original_position, patrol.patrol_target
                            ));
                        }
                        Order::Work(target) => {
                            ui.label(format!("Going to work at {:?}", target));
                        }
                    }
                };
                ui.separator();
            }
        });
}

fn draw_bottom_panel(mut contexts: EguiContexts, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    egui::Window::new("Bottom Panel")
        // .anchor(Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
        .collapsible(false)
        // .default_width(window.width())
        .fixed_size(egui::Vec2::new(100.0, 100.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Bottom Panel");
        });
}

fn draw_patrol_path(mut gizmos: Gizmos, units: Query<(&Order, &GlobalTransform)>) {
    for (order, transform) in &units {
        match order {
            Order::Move(target) => {
                gizmos.arrow(transform.translation(), *target, Color::WHITE);
            }
            Order::Patrol(patrol_details) => gizmos.line(
                patrol_details.original_position,
                patrol_details.patrol_target,
                Color::WHITE,
            ),
            _ => {}
        }
    }
}
