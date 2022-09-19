use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(crate::enemy::EnemyPlugin);
        app.add_plugin(crate::tower::TowerPlugin);
    }
}