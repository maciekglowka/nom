use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque}
};

pub mod actions;
mod action_modifiers;
mod board;
pub mod components;
pub mod globals;
mod resources;
mod systems;

pub use board::Board;
pub use resources::{PlayerResources, Resource};

use action_modifiers::ActionModifier;

pub struct GameSetup {
    pub action_modifiers: HashMap<TypeId, Vec<ActionModifier>>
}
impl GameSetup {
    pub fn new() -> Self {
        GameSetup { action_modifiers: HashMap::new() }
    }
}

pub fn init(world: &mut World) -> GameSetup {
    let board = board::Board::new();
    world.insert_resource(board);
    board::init_board(world);

    world.insert_resource(actions::ActionQueue(VecDeque::new()));

    let mut resources = resources::PlayerResources::new();
    resources.add_resources(&HashMap::from_iter([
        (resources::Resource::Food, 50),
        (resources::Resource::Energy, 20),
    ]));
    world.insert_resource(resources);

    let _ = systems::spawn_with_position(
        world,
        "Player",
        Vector2I::new(globals::BOARD_WIDTH as i32 / 2, 0)
    );
    let mut setup = GameSetup::new();
    register_action_modifiers(&mut setup);
    setup
}

pub fn game_step(world: &mut World, state: &GameSetup) {
    systems::execute_action(world, state);
}

fn register_action_modifiers(setup: &mut GameSetup) {
    // setup.action_handlers.insert(
    //     TypeId::of::<actions::ShiftBoard>(),
    //     vec![action_handlers::dummy_shift_handler]
    // );
}