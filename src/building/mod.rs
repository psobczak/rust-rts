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
            .register_type::<BuildingCapacity>()
            .add_systems(Startup, spawn_gold_mine)
            .add_systems(Update, (print_gold_count, assign_worker));
    }
}

#[derive(Component)]
struct Building;

#[derive(Component, Debug, Reflect)]
struct GoldMine {
    pub gold_left: u32,
}

#[derive(Component, Debug, Reflect)]
pub struct BuildingCapacity {
    pub max: u32,
    pub units: Vec<Entity>,
}

#[derive(Component, Debug, Reflect)]
pub struct WorkingInMine;

fn spawn_gold_mine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        BuildingCapacity {
            max: 5,
            units: vec![],
        },
        Name::from("Gold Mine"),
        Building,
        GoldMine { gold_left: 10_000 },
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
    mut gold_mines: Query<(Entity, &mut BuildingCapacity, &GlobalTransform), With<GoldMine>>,
    selected_workers: Query<Entity, (With<Unit>, With<Selected>, With<Worker>)>,
) {
    for worker in &selected_workers {
        for (mine_entity, mut capacity, transform) in &mut gold_mines {
            for event in reader.read() {
                if event.target == mine_entity
                    && !capacity.units.contains(&worker)
                    && capacity.units.len() < capacity.max as usize
                {
                    capacity.units.push(worker);
                    commands
                        .entity(worker)
                        .insert(Order::Work(transform.translation()));
                }
            }
        }
    }
}
