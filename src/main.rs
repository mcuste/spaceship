mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpacehipPlugin;

fn main() {
    App::new()
        // Built-ins
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpacehipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
