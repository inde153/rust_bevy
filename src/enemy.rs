use bevy::prelude::*;
use rand::Rng;


// 적 컴포넌트 정의
#[derive(Component)]
pub struct Enemy;
use crate::player::Player;

const SPAWN_INTERVAL: f32 = 2.0; // 적 생성 간격 (초)

// 적을 주기적으로 생성하는 시스템
pub fn spawn_enemy_system(
    time: Res<Time>,            // 게임 시간
    mut commands: Commands,    // 엔티티 생성 명령
    mut timer: Local<Timer>,   // 로컬로 타이머 상태 관리
) {
    // 타이머 갱신
    timer.tick(time.delta());

    // 타이머가 만료되면 적을 생성
    if timer.finished() {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(-500.0..500.0); // 화면 좌우 범위
        let random_y = rng.gen_range(-500.0..500.0); // 화면 상하 범위

        commands.spawn((
            Enemy, // 적 컴포넌트
            Sprite {
                color: Color::srgb(1.0,0.0,0.0), // 색상
                custom_size: Some(Vec2::new(50.0, 50.0)), // 크기 설정
                ..Default::default()
            },
            Transform::from_xyz(random_x, random_y, 0.0), // 화면 밖에서 생성
        ));
        *timer = Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Once); 
    }
}

pub fn move_towards_player_system(
    mut query_enemy: Query<(&mut Transform, &Enemy), Without<Player>>, // 플레이어를 제외한 적만
    query_player: Query<&Transform, With<Player>>, // 플레이어의 위치
) {
    // 플레이어가 존재하는지 확인
    if let Some(player_transform) = query_player.iter().next() {
        // 적들의 위치를 업데이트
        for (mut transform, _) in query_enemy.iter_mut() {
            // 적의 위치에서 플레이어로 가는 방향 벡터 계산
            let direction = player_transform.translation - transform.translation;

            // 이동 방향 정규화
            let direction = direction.normalize();

            // 적의 이동 속도 설정
            let speed = 100.0; // 이동 속도

            // 적의 위치를 플레이어 쪽으로 이동
            transform.translation += direction * speed * 1.0 / 60.0; // 60fps 기준으로 프레임에 맞게 이동
        }
    } else {
        eprintln!("플레이어가 존재하지 않습니다.");
    }
}

