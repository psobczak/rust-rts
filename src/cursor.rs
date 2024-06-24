use bevy::prelude::*;
use bevy_mod_picking::events::{Move, Pointer};
use bevy_rts_camera::Ground;

pub struct CursorPlugin;

#[derive(Resource)]
pub struct MousePosition {
    pub world: Vec3,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition { world: Vec3::ZERO })
            .add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut reader: EventReader<Pointer<Move>>,
    mut mouse_position: ResMut<MousePosition>,
    ground: Query<(Entity, &GlobalTransform), With<Ground>>,
) {
    let (ground, ground_transform) = ground.single();
    let (camera, camera_transform) = camera.single();

    for event in reader.read() {
        if ground == event.target {
            if let Some(ray) =
                camera.viewport_to_world(camera_transform, event.pointer_location.position)
            {
                let Some(distance) = ray.intersect_plane(
                    ground_transform.translation(),
                    Plane3d::new(ground_transform.up()),
                ) else {
                    return;
                };

                let point = ray.get_point(distance);
                mouse_position.world = point;
            }
        }
    }
}
