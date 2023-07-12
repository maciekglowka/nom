use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;

pub mod actions;
mod board;
pub mod components;
pub mod globals;
mod systems;

pub use board::Board;

pub fn init(world: &mut World) {
    let board = board::Board::new();
    world.insert_resource(board);
    board::init_board(world);

    world.insert_resource(actions::CurrentAction(None));

    let player = world.spawn_entity();
    let _ = world.insert_component(player, components::Player);
    let _ = world.insert_component(player, components::Position(
        Vector2I::new(globals::BOARD_WIDTH as i32 / 2, 0)
    ));
    let _ = world.insert_component(player, components::Name("Player".into()));
}

pub fn game_step(world: &mut World) {
    systems::execute_action(world);
}