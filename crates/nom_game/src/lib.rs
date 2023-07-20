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
pub mod input;
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

    world.insert_resource(input::GameInput::default());

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
        return;
    }
    if let Some(input) = input::get_current_input(world) {
        input::handle_input(world, input);
        return;
    }
    if let Some(mut game_input) = world.get_resource_mut::<input::GameInput>() {
        // still waiting for input - return
        if game_input.required.is_some() { return }
        // otherwise if nothing to do and no input is required ask for tile movement
        game_input.required = Some(input::InputRequired::Tile);
    }

}

fn register_action_modifiers(manager: &mut GameManager) {
    manager.action_modifiers = HashMap::from_iter([
        (
            TypeId::of::<actions::MovePlayer>(), vec![
                action_modifiers::movement_cost_modifier,
                action_modifiers::shift_board_modifier,
                action_modifiers::movement_enter_tile_modifier,
            ]
        ),
        (
            TypeId::of::<actions::EnterTile>(), vec![
                action_modifiers::enter_tile_resources_modifier,
                action_modifiers::enter_tile_chest_modifier
            ]
        )
    ]);
}
