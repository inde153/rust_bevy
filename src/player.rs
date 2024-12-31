use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// 플레이어 컴포넌트
#[derive(Component)]
pub struct Player {
    pub health: f32, // 플레이어 체력
    pub max_health: f32,
}

#[derive(Component)]
pub struct HealthBar {
    pub player: Entity,
}

impl HealthBar {
    pub fn new(player: Entity) -> Self {
        HealthBar { player }
    }
}

pub fn spawn_player(commands: &mut Commands, position: Vec3) {
    let player_entity = commands.spawn((
        Player {
            health: 100.0,
            max_health: 100.0,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 1.0), // 플레이어 색상
            custom_size: Some(Vec2::new(50.0, 50.0)), // 크기 설정
            ..Default::default()
        },
        Transform::from_translation(position),
    )).id();

    commands.spawn((
        HealthBar::new(player_entity),
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // 체력바 색상
            custom_size: Some(Vec2::new(40.0, 5.0)), // 기본 체력바 크기
            ..Default::default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 40.0, 0.0)), // 플레이어 아래에 위치
    ));
}

pub fn update_health_bar(
    mut health_bar_query: Query<(&HealthBar, &mut Sprite, &mut Transform)>,
    player_query: Query<(&Player, &Transform), Without<HealthBar>>, // HealthBar를 제외한 Player 쿼리
) {
    for (health_bar, mut sprite, mut transform) in health_bar_query.iter_mut() {
        if let Ok((player, player_transform)) = player_query.get(health_bar.player) {
            // 체력바의 위치를 플레이어 아래로 업데이트
            transform.translation = Vec3::new(
                player_transform.translation.x, // 플레이어의 x 위치
                player_transform.translation.y + 40.0, // 플레이어 아래에 위치
                transform.translation.z,
            );

            // 체력바의 크기를 플레이어의 체력에 맞게 조정
            let health_ratio = player.health / player.max_health;
            let new_width = 50.0 * health_ratio; // 체력 비율에 따라 너비 조정
            sprite.custom_size = Some(Vec2::new(new_width, 5.0)); // 체력에 맞게 크기 조정
        }
    }
}



pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>, // 키보드 입력
    mut query: Query<(&mut Transform, &Player)>, // 플레이어 쿼리
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // 키 입력에 따른 방향 설정
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // 속도 적용
        let speed = 300.0;
        transform.translation += direction.normalize_or_zero() * speed * TIME_STEP;
    }
}