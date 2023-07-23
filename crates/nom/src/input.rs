use macroquad::prelude::*;
use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::World;

use nom_graphics::{
    ui::{ButtonState, InputState}
};

pub fn get_ui_state(camera: &Camera2D) -> InputState {
    // use event streams ?
    let mut left = ButtonState::Up;
    if is_mouse_button_down(MouseButton::Left) {
        left = ButtonState::Down
    }
    if is_mouse_button_released(MouseButton::Left) {
        left = ButtonState::Released
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        left = ButtonState::Pressed
    }
    InputState {
        mouse_screen_position: get_mouse_screen_position(camera),
        mouse_world_position: get_mouse_world_position(camera),
        mouse_button_left: left
    }
}

fn get_mouse_screen_position(camera: &Camera2D) -> Vector2F {
    let v = mouse_position();
    Vector2F::new(v.0, v.1)
}

fn get_mouse_world_position(camera: &Camera2D) -> Vector2F {
    let mouse = mouse_position();
    let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
    Vector2F::new(v.x, v.y)
}

// pub fn set_input_action(camera: &Camera2D, manager: &mut GameManager) {
//     if is_mouse_button_released(MouseButton::Left) {
//         match manager.input_required {
//             Some(InputRequired::Tile) => {
//                 manager.current_input = Some(Input::Tile(get_mouse_tile(camera)))
//             },
//             _ => ()
//         }
//     }
// }