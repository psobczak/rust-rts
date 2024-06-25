pub mod worker;

use bevy::prelude::*;

pub const UNIT_SIZE: f32 = 0.5;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, worker::spawn_workers)
            .add_systems(Update, worker::hide_workers_in_gold_mine);
    }
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Worker;
