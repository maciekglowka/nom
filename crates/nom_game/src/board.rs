use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};
use rand::prelude::*;
use std::collections::VecDeque;

use crate::globals::{BOARD_WIDTH, BOARD_LENGTH};
use crate::systems::spawn_with_position;

pub struct Board {
    pub tiles: VecDeque<Vec<Entity>>,
    pub shift: usize
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: VecDeque::new(), shift: 0 }
    }
}

pub fn init_board(world: &mut World) {
    for _ in 0..BOARD_LENGTH {
        spawn_row(world);
    }
}

pub fn spawn_row(world: &mut World) {
    let mut rng = thread_rng();

    let shift = match world.get_resource::<Board>() {
        Some(b) => b.shift,
        _ => return
    };
    let mut row = Vec::new();
    for x in 0..BOARD_WIDTH {
        let v = Vector2I::new(x as i32, shift as i32);

        let kind = match rng.gen_range(0.0..1.0) {
            a if a < 0.2 => "Crate",
            a if a < 0.5 => "Plains",
            a if a < 0.75 => "Forest",
            _ => "Fields"
        };

        let Some(entity) = spawn_with_position(world, kind, v) else { continue };
        row.push(entity);
    }
    if let Some(mut board) = world.get_resource_mut::<Board>() {
        board.tiles.push_back(row);
        board.shift += 1;
    }
}