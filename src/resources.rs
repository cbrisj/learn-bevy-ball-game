use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0; // 星星生成time
pub const ENEMY_SPAWN_TIME: f32 = 5.0;  // 敌人生成time

/// 声明为资源
/// insert_resource(Score{value:0})
/// 这只是其中一个插入资源的方法，并且可能不合适，因为会被覆盖
/// init_resource::<Score>() 建议
#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

/// 为资源Score 实现Default trait
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { 
            scores: Vec::new() 
        }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer { 
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

/// 敌人生成计时器
#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { 
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}