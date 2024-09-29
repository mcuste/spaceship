use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, CAMERA_DISTANCE, 0.0))
            .looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
