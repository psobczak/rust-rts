use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts,
};

use crate::{order::Order, selection::Selected};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_example_system);
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
                    }
                };
                ui.separator();
            }
        });
}
