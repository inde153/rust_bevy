use bevy::prelude::*;
mod player;  
mod enemy;  
mod setup;   

use crate::setup::setup;
use crate::player::{ player_movement,update_health_bar };
use crate::enemy::{move_towards_player_system,spawn_enemy_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup) // 기동 시
        .add_systems(Update, player_movement) // 매 프레임 업데이트
        .add_systems(Update,spawn_enemy_system)
        .add_systems(Update, move_towards_player_system)
        .add_systems(Update, update_health_bar)
        .run();
}