use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

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
/// .add_systems((player_movement, confine_player_movement.after(player_movement)))
/// 还可以使用 .add_systems((player_movement, confine_player_movement).chain())
/// 过度使用会损失性能
/// 因为这两个系统都是对player变换的可变访问，无论如何排序，这两个系统都不会并行执行，因此此处排序无性能损失
impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            // Startup Systems
            // .add_startup_system(spawn_player)
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // Systems
            // .add_system(
            //     player_movement
            //         .in_set(PlayerSystemSet::Movement)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            // .add_system(
            //     confine_player_movement
            //         .in_set(PlayerSystemSet::Confinement)
            //         .run_if(in_state(AppState::Game))
            //         .run_if(in_state(SimulationState::Running)),
            // )
            // .add_system(enemy_hit_player)
            // .add_system(player_hit_star)
            .add_systems(
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
