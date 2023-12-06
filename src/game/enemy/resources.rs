use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 5.0; // 敌人生成time

/// 敌人生成计时器
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}