use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy::app::AppExit;

pub const PLAYER_SPEED: f32 = 500.0; // 初始速度
pub const PLAYER_SIZE: f32 = 64.0; // 初始大小
pub const NUMBER_OF_ENEMIES: usize = 4; // 初始敌人数量 4
pub const ENEMY_SPEED: f32 = 200.0; // 敌人初始速度
pub const ENEMY_SIZE: f32 = 64.0; // 初始大小
pub const NUMBER_OF_STARS: usize = 10; // 初始星星数量
pub const STAR_SIZE: f32 = 30.0; // 星星大小
pub const STAR_SPAWN_TIME: f32 = 1.0; // 星星生成time
pub const ENEMY_SPAWN_TIME: f32 = 5.0;  // 敌人生成time

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<HighScores>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<EnemySpawnTimer>()
    .add_event::<GameOver>()
    .add_startup_systems((spawn_camera, spawn_player, spawn_enemies))
    .add_startup_system(spawn_stars)
    .add_system(player_movement)
    .add_system(confine_player_movement)
    .add_system(enemy_movement)
    .add_system(update_enemy_direction)
    .add_system(confine_enemy_movement)
    .add_system(enemy_hit_player)
    .add_system(player_hit_star)
    .add_system(update_score)
    .add_system(tick_star_spawn_timer)
    .add_system(spawn_stars_over_time)
    .add_system(tick_enemy_spawn_timer)
    .add_system(spawn_enemies_over_time)
    .add_system(exit_game)
    .add_system(handle_game_over)
    .add_system(update_high_scores)
    .add_system(high_scores_updated)
    .run();
}


#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

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

/// 自定义游戏结束事件
pub struct GameOver {
    pub score: u32,
}

/// Commands
/// Query<&Window, With<PrimaryWindow>> 查询窗体状态
/// Res<AssetServer> asset资源服务器
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // get_single()获取单个返回结果类型
    let window = window_query.get_single().unwrap();

    // 添加精灵束
    commands.spawn(
      (
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
      )  
    );
}

/// 添加一个2D相机Bundle
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}
/// 添加敌人
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) { 
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                   direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    }
}

/// 添加星星
/// SpriteBundle{} 精灵包
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn(
          (
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
          )  
        );
    }
}

/// Res<Input<KeyCode>>, 获取键盘输入
/// Query<&mut Transform, With<Player>> 查询对带有Player组件的转换的可变引用
/// Res<Time> 时间资源
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0,1.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0,-1.0,0.0);
        }
        // 检查方向向量的长度是否大于零，以确保我们的对角线不会移动得比只是垂直或水平访问
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        // 方向 * 速度 * 时间增量 这样就可以与帧率无关
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/// 限制player 超过屏幕
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if  translation.x > x_max {
            translation.x = x_max;
        }

        if  translation.y < y_min {
            translation.y = y_min;
        } else if  translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

/// 敌人移动
/// 查询多个资源，需要包含在元组里
pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        // 方向 * 速度 * 时间增量 这样就可以与帧率无关
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();

    }
}

/// 碰到边缘回弹
/// audio: Res<Audio> 音频资源
pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min= 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0+half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_chenged = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_chenged = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_chenged = true;
        }

        // 如果方向改变了，加载音效
        if direction_chenged {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            audio.play(sound_effect);
        }
    }
}

/// 限制敌人在屏幕内
pub fn confine_enemy_movement(
    mut enmey_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min= 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0+half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enmey_query.iter_mut() {
        let mut translation = transform.translation;
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }

}

/// Query<(Entity, &Transform), With<Player>>
/// 查询实体本身
/// commands.entity(player_entity).despawn();
/// 当敌人与玩家碰撞时，需要去销毁玩家实体，所以需要查询出来player_entity
pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_write: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,

) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_write.send(GameOver { score: score.value });
            }
        }
    }
}

/// 玩家碰撞星星
pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                score.value += 1;
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

/// 打印更新后的score
pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
            
        
    }
}

/// 更新敌人计时器
pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

/// 随时间生成敌人
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

/// 游戏退出事件
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores.scores.push(("Player".to_string(), event.score));        
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>,
) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}