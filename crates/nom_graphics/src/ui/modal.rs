use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use nom_game::input::{GameInput, Input, InputRequired};

use super::{UiState, GraphicsBackend, SpriteColor};
use super::buttons::Button;

pub fn draw_modal(
    game_input: &mut GameInput,
    backend: &dyn GraphicsBackend,
    state: &UiState,
) {
    let viewport_size = backend.viewport_size();
    let mut clicked = None;
    if let Some(InputRequired::Action(options)) = &mut game_input.required {
        for (i, entry) in options.iter_mut().enumerate() {
            if Button::new(
                viewport_size.x / 2. - 200.,
                viewport_size.y / 2. - 50. + i as f32 * 50.,
                400.,
                40.,
            )
            .with_text(
                &entry.0,
                SpriteColor(0, 0, 0, 255),
                32
            )
            .with_color(SpriteColor(255, 255, 255, 255))
            .draw(backend)
            .clicked(state) { 
                clicked = entry.1.take(); 
            }
        }
    }
    if let Some(clicked) = clicked {
        game_input.current = Some(Input::Action(clicked));
        game_input.required = None;
    }
}