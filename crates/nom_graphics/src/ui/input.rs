use rogalik::storage::World;

use nom_game::input::{GameInput, Input, InputRequired};

use crate::graphics::world_to_tile;
use super::{ButtonState, GraphicsBackend, UiState};
use super::modal::draw_modal;

pub fn handle_input(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    state: &UiState
) {
    let Some(mut game_input) = world.get_resource_mut::<GameInput>() else { return };;
    match game_input.required {
        Some(InputRequired::Tile) => handle_tile_input(&mut game_input, state),
        Some(InputRequired::Action(_)) => handle_action_input(
            &mut game_input,
            state,
            backend
        ),
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

fn handle_action_input(
    game_input: &mut GameInput,
    state: &UiState,
    backend: &dyn GraphicsBackend
) {
    draw_modal(game_input, backend, state);
}