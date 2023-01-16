use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}
