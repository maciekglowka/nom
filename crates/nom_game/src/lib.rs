use rogalik::{
    math::vectors::Vector2I,
    storage::World
};
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque}
};

pub mod actions;
mod action_modifiers;
mod board;
pub mod components;
pub mod events;
pub mod globals;
mod resources;
mod systems;

pub use board::Board;
pub use resources::{PlayerResources, Resource};

use action_modifiers::ActionModifier;

#[derive(Default)]
pub struct GameManager {
    pub action_modifiers: HashMap<TypeId, Vec<ActionModifier>>
}

pub fn init(world: &mut World) -> GameManager {
    let board = board::Board::new();
    world.insert_resource(board);
    board::init_board(world);

    world.insert_resource(events::Events::new());

    // world.insert_resource(input::GameInput::default());

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
    let mut manager = GameManager::default();
    register_action_modifiers(&mut manager);
    manager
}

pub fn game_step(world: &mut World, manager: &GameManager) {
    if let Some(action) = systems::get_current_action(world) {
        systems::execute_action(action, world, manager);
    }

}

fn register_action_modifiers(manager: &mut GameManager) {
    manager.action_modifiers = HashMap::from_iter([
        (
            TypeId::of::<actions::MovePlayer>(), [
                action_modifiers::movement_cost_modifier,
                action_modifiers::shift_board_modifier,
                action_modifiers::movement_enter_tile_modifier,
            ].to_vec()
        ),
        (
            TypeId::of::<actions::EnterTile>(), [
                action_modifiers::enter_tile_resources_modifier,
                action_modifiers::enter_tile_chest_modifier
            ].to_vec()
        )
    ]);
}
