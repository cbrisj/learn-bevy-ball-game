use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

// /// 声明为系统集
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;

// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct ConfinementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayPlugin;

/// player_movement.before(confine_player_movement)
/// confine_player_movement.after(player_movement)
/// 两者等价
///
/// 还可以使用 .add_systems((player_movement, confine_player_movement).chain())
impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_startup_system(spawn_player)
            .add_system(player_movement.in_set(PlayerSystemSet::Movement))
            .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
