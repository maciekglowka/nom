use rogalik::storage::World;

use nom_game::actions;

use crate::graphics::world_to_tile;
use super::{ButtonState, GraphicsBackend, InputState, UiState, UiMode};
use super::modal::{ModalData, draw_modal};

pub fn handle_input(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    input_state: &InputState,
    ui_state: &mut UiState,
) {
    let complete = match ui_state.mode {
        UiMode::Tile => handle_tile_input(world, input_state),
        UiMode::Modal(ref mut data) => handle_modal_input(world, backend, input_state, data)
    };
    if complete {
        ui_state.mode = UiMode::Tile
    }
}

fn handle_tile_input(world: &mut World, input_state: &InputState) -> bool {
    if input_state.mouse_button_left != ButtonState::Pressed {
        return false;
    }
    let v = world_to_tile(input_state.mouse_world_position);
    if let Some(mut queue) = world.get_resource_mut::<actions::ActionQueue>() {
        queue.0.push_back(Box::new(actions::MovePlayer { target: v }));
    }
    true
}

fn handle_modal_input(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    input_state: &InputState,
    data: &mut ModalData
) -> bool{
    let choice = draw_modal(backend, input_state, data);
    if let Some(choice) = choice {
        if let Some(mut queue) = world.get_resource_mut::<actions::ActionQueue>() {
            if let Some(action) = data.choices[choice].1.take() {
                queue.0.push_back(action);
            }
        }
        return true
    }
    false
}
