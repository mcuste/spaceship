use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("models/spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..Default::default()
        },
    });
}

pub struct SpacehipPlugin;

impl Plugin for SpacehipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}
