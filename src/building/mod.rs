use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    PickableBundle,
};

use crate::{
    order::Order,
    selection::Selected,
    unit::{Unit, Worker},
};

const GOLD_MINE_SIZE: f32 = 1.0;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GoldMine>()
            .add_systems(Startup, spawn_gold_mine)
            .add_systems(Update, (print_gold_count, assign_worker));
    }
}

#[derive(Component)]
struct Building;

#[derive(Component, Debug, Reflect)]
struct GoldMine {
    pub max_workers: u32,
    pub workers: Vec<Entity>,
    pub gold_left: u32,
}

#[derive(Component, Debug, Reflect)]
pub struct WorkingInMine;

fn spawn_gold_mine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::from("Gold Mine"),
        Building,
        GoldMine {
            max_workers: 5,
            workers: vec![],
            gold_left: 10_000,
        },
        PickableBundle::default(),
        PbrBundle {
            material: materials.add(Color::GOLD),
            mesh: meshes.add(Cuboid::from_size(Vec3::splat(GOLD_MINE_SIZE))),
            transform: Transform::from_xyz(0.0, GOLD_MINE_SIZE / 2.0, 0.0),
            ..Default::default()
        },
    ));
}

fn print_gold_count(
    mut reader: EventReader<Pointer<Click>>,
    gold_mines: Query<(Entity, &GoldMine)>,
) {
    for event in reader.read() {
        for (entity, mine) in &gold_mines {
            if event.target == entity {
                info!("Gold left: {}", mine.gold_left)
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn assign_worker(
    mut commands: Commands,
    mut reader: EventReader<Pointer<Click>>,
    mut gold_mines: Query<(Entity, &mut GoldMine, &GlobalTransform)>,
    selected_workers: Query<Entity, (With<Unit>, With<Selected>, With<Worker>)>,
) {
    for worker in &selected_workers {
        for (mine_entity, mut gold_mine, transform) in &mut gold_mines {
            for event in reader.read() {
                if event.target == mine_entity
                    && !gold_mine.workers.contains(&worker)
                    && gold_mine.workers.len() < gold_mine.max_workers as usize
                {
                    gold_mine.workers.push(worker);
                    commands
                        .entity(worker)
                        .insert(Order::Work(transform.translation()));
                }
            }
        }
    }
}
