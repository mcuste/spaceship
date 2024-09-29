use bevy::prelude::*;

use crate::spaceship::Velocity;

fn update_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        transform.translation.z += velocity.value.z * time.delta_seconds();
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_positions);
    }
}
