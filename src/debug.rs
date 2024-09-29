use bevy::prelude::*;

fn print_positions(query: Query<(Entity, &Transform)>) {
    // Log the entity ID and position of each entity with a Position component
    for (entity, transform) in query.iter() {
        println!(
            "Entity {:?} is at positon {:?}",
            entity, transform.translation
        );
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_positions);
    }
}
