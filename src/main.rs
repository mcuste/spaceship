mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpacehipPlugin;

const BACKGROUND_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

fn main() {
    App::new()
        // Built-ins
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(AmbientLight::default())
        .add_plugins(AsteroidsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpacehipPlugin)
        .run();
}
