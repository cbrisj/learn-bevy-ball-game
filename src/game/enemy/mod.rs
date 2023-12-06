use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub const ENEMY_SIZE: f32 = 64.0; // 初始大小
pub const ENEMY_SPEED: f32 = 200.0; // 敌人初始速度
pub const NUMBER_OF_ENEMIES: usize = 4; // 初始敌人数量 4

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            // .add_startup_system(spawn_enemies)
            // 进入状态系统
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // .add_system(enemy_movement)
            // .add_system(update_enemy_direction)
            // .add_system(confine_enemy_movement)
            // .add_system(tick_enemy_spawn_timer)
            // .add_system(spawn_enemies_over_time)
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // 推出状态系统
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
