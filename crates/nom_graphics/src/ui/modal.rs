use rogalik::{
    math::vectors::Vector2F,
    storage::World
};

use nom_game::actions::Action;

use super::{InputState, UiState, GraphicsBackend, SpriteColor};
use super::buttons::Button;
use super::span::Span;

pub struct ModalData<'a> {
    pub text: String,
    pub choices: Vec<(Span<'a>, Option<Box<dyn Action>>)>
}

pub fn draw_modal(
    backend: &dyn GraphicsBackend,
    input_state: &InputState,
    data: &ModalData
) -> Option<usize> {
    let viewport_size = backend.viewport_size();
    let mut clicked = None;

    for (i, entry) in data.choices.iter().enumerate() {
        let button = Button::new(
                viewport_size.x / 2. - 200.,
                viewport_size.y / 2. - 50. + i as f32 * 50.,
                400.,
                40.,
            )
            .with_span(&entry.0)
            .with_color(SpriteColor(50, 50, 50, 255));
        button.draw(backend);
        if button.clicked(input_state) { 
            clicked = Some(i);
        }
    }
    clicked
}