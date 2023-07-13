use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;
use std::collections::{HashMap, VecDeque};

pub mod actions;
mod board;
pub mod components;
pub mod globals;
mod resources;
mod systems;

pub use board::Board;
pub use resources::{PlayerResources, Resource};

pub fn init(world: &mut World) {
    let board = board::Board::new();
    world.insert_resource(board);
    board::init_board(world);

    world.insert_resource(actions::ActionQueue(VecDeque::new()));

    let mut resources = resources::PlayerResources::new();
    resources.add(HashMap::from_iter([
        (resources::Resource::Food, 50),
        (resources::Resource::Energy, 20),
    ]));
    world.insert_resource(resources);

    let _ = systems::spawn_with_position(
        world,
        "Player",
        Vector2I::new(globals::BOARD_WIDTH as i32 / 2, 0)
    );
}

pub fn game_step(world: &mut World) {
    systems::execute_action(world);
}