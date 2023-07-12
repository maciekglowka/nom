use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;

use nom_data::GameData;

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

    // let player = world.spawn_entity();
    // let _ = world.insert_component(player, components::Position(
    //     Vector2I::new(globals::BOARD_WIDTH as i32 / 2, 0)
    // ));
    // let _ = world.insert_component(player, components::Name("Player".into()));
    // let player_data = world.get_resource::<GameData>().unwrap()
    //     .entities.get("Tile").unwrap().clone();
    // components::insert_data_components(player, world, &player_data.components);

    let _ = systems::spawn_with_position(
        world,
        "Player",
        Vector2I::new(globals::BOARD_WIDTH as i32 / 2, 0)
    );
}

pub fn game_step(world: &mut World) {
    systems::execute_action(world);
}