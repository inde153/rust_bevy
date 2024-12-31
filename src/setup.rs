use bevy::prelude::*;
use bevy::ui::{Val};
use crate::{player::{spawn_player}};  // player.rs에서 Player 구조체를 가져옵니다.

pub fn setup(mut commands: Commands) {
    // 카메라 생성
    commands.spawn(Camera2d::default()); // 2D 카메라 설정

    // 초기 플레이어 스폰
    spawn_player(&mut commands, Vec3::ZERO);
}
