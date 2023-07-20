use rogalik::math::vectors::Vector2F;
use rogalik::storage::World;

use nom_data::SpriteColor;
use nom_game::{GameManager, PlayerResources};

use super::GraphicsBackend;

mod input;
mod buttons;
mod modal;

#[derive(Default)]
pub struct UiState {
    pub mouse_world_position: Vector2F,
    pub mouse_screen_position: Vector2F,
    pub mouse_button_left: ButtonState
}

#[derive(Default, PartialEq)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn ui_update(
    world: &mut World,
    backend: &dyn GraphicsBackend,
    ui_state: &UiState,
) {
    draw_status(world, backend);
    input::handle_input(world, backend, ui_state);
}

fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let Some(resources) = world.get_resource::<PlayerResources>() else { return };
    for (i, (k, v)) in resources.stock.iter().enumerate() {
        backend.draw_ui_text(
            "default", 
            &format!("{}: {}", k, v), 
            Vector2F::new(10., 30. + i as f32 * 30.),
            24, 
            SpriteColor(255, 255, 255, 255)
        );
    }
}