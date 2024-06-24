use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    PickableBundle,
};

const GOLD_MINE_SIZE: f32 = 1.0;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_gold_mine)
            .add_systems(Update, print_gold_count);
    }
}

#[derive(Component)]
struct Building;

#[derive(Component)]
struct GoldMine(u32);

fn spawn_gold_mine(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Building,
        GoldMine(10_000),
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
                info!("Gold left: {}", mine.0)
            }
        }
    }
}
