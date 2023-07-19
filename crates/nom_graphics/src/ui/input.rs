use rogalik::storage::World;

use nom_game::input::{GameInput, Input, InputRequired};

use crate::graphics::world_to_tile;
use super::{ButtonState, UiState};

pub fn handle_input(
    world: &mut World,
    state: &UiState
) {
    let Some(mut game_input) = world.get_resource_mut::<GameInput>() else { return };;
    match game_input.required {
        Some(InputRequired::Tile) => handle_tile_input(&mut game_input, state),
        _ => ()
    }
}

fn handle_tile_input(game_input: &mut GameInput, state: &UiState) {
    if state.mouse_button_left != ButtonState::Pressed {
        return;
    }
    game_input.current = Some(
        Input::Tile(world_to_tile(state.mouse_world_position))
    );
}