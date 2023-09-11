use bevy::prelude::*;

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
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: Vec::new() }
    }
}
