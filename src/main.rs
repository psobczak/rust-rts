mod building;
mod order;
mod selection;
mod unit;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rts_camera::{Ground, RtsCamera, RtsCameraControls, RtsCameraPlugin};
use building::BuildingPlugin;
use order::OrderPlugin;
use selection::SelectionPlugin;
use unit::UnitPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty RTS".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            DefaultPickingPlugins,
            RtsCameraPlugin,
            SelectionPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        ))
        .add_plugins((BuildingPlugin, UnitPlugin, OrderPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RtsCamera::default(),
        RtsCameraControls::default(),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Ground,
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
            material: materials.add(Color::WHITE),
            ..default()
        },
    ));
}
