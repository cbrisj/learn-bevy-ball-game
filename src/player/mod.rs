use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayPlugin;

/// player_movement.before(confine_player_movement)
/// confine_player_movement.after(player_movement)
/// 两者等价
/// 
/// 还可以使用 .add_systems((player_movement, confine_player_movement).chain())
impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_systems((player_movement, confine_player_movement).chain())
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
