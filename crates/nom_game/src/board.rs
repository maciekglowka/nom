use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};
use rand::prelude::*;
use std::collections::VecDeque;

use nom_data::GameData;

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
    let shift = match world.get_resource::<Board>() {
        Some(b) => b.shift,
        _ => return
    };

    let choices = if let Some(game_data) = world.get_resource::<GameData>() {
        game_data.tiles.iter()
            .filter_map(|s| match game_data.entities.get(s) {
                Some(data) => if data.min_distance.unwrap_or(0) <= shift {
                    Some((s.clone(), data.spawn_chance))
                } else {
                    None
                },
                None => None
            })
            .filter_map(|e| match e.1 {
                Some(chance) => Some((e.0, chance)),
                None => None
            })
            .collect::<Vec<_>>()
    } else {
        return
    };
    
    let mut row = Vec::new();
    for x in 0..BOARD_WIDTH {
        let v = Vector2I::new(x as i32, shift as i32);

        let kind = get_random_tile(&choices);

        let Some(entity) = spawn_with_position(world, kind, v) else { continue };
        row.push(entity);
    }
    if let Some(mut board) = world.get_resource_mut::<Board>() {
        board.tiles.push_back(row);
        board.shift += 1;
    }
}

fn get_random_tile<'a>(choices: &'a Vec<(String, f32)>) -> &'a str {
    let mut rng = thread_rng();
    &choices.choose_weighted(&mut rng, |a| a.1).unwrap().0
}