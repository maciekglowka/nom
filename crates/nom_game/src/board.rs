use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};
use std::collections::VecDeque;

use crate::components::{Name, Position, Tile};
use crate::globals::{BOARD_WIDTH, BOARD_LENGTH};

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
    let shift = match world.get_resource::<Board>() {
        Some(b) => b.shift,
        _ => return
    };
    let mut row = Vec::new();
    for x in 0..BOARD_WIDTH {
        let v = Vector2I::new(x as i32, shift as i32);
        let entity = world.spawn_entity();
        let _ = world.insert_component(entity, Name("Tile".into()));
        let _ = world.insert_component(entity, Position(v));
        let _ = world.insert_component(entity, Tile);
        row.push(entity);
    }
    if let Some(mut board) = world.get_resource_mut::<Board>() {
        board.tiles.push_back(row);
        board.shift += 1;
    }
}