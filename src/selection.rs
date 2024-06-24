use bevy::prelude::*;
use bevy_mod_picking::{
    events::Pointer,
    selection::{Deselect, Select},
};

use crate::unit::Unit;

pub struct SelectionPlugin;

#[derive(Component)]
pub struct Selected;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_selection, handle_deselection));
    }
}

fn handle_selection(
    mut commands: Commands,
    mut reader: EventReader<Pointer<Select>>,
    units: Query<Entity, (With<Unit>, Without<Selected>)>,
) {
    for event in reader.read() {
        for unit in &units {
            if event.target == unit {
                commands.entity(unit).insert(Selected);
            }
        }
    }
}

fn handle_deselection(
    mut commands: Commands,
    mut reader: EventReader<Pointer<Deselect>>,
    units: Query<Entity, (With<Unit>, With<Selected>)>,
) {
    for event in reader.read() {
        for unit in &units {
            if event.target == unit {
                commands.entity(unit).remove::<Selected>();
            }
        }
    }
}
