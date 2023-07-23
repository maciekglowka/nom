use rogalik::{
    events::SubscriberHandle,
    math::vectors::Vector2F,
    storage::World
};
use std::collections::VecDeque;

use nom_data::SpriteColor;
use nom_game::{
    actions,
    events::{Events, ResourceChangeEvent, ChestEvent},
    PlayerResources, Resource
};

use super::GraphicsBackend;

mod input;
mod buttons;
mod events;
mod modal;
mod span;

#[derive(Default)]
pub struct InputState {
    pub mouse_world_position: Vector2F,
    pub mouse_screen_position: Vector2F,
    pub mouse_button_left: ButtonState
}

pub enum UiMode<'a> {
    Tile,
    Modal(modal::ModalData<'a>)
}

pub struct UiState<'a> {
    pub ev_resource: SubscriberHandle<ResourceChangeEvent>,
    pub ev_chest: SubscriberHandle<ChestEvent>,
    pub mode: UiMode<'a>,
    pub bubbles: VecDeque<(f32, span::Span<'a>)>
}
impl<'a> UiState<'a> {
    pub fn new(world: &mut World) -> Self {
        let mut events = world.get_resource_mut::<Events>().unwrap();
        UiState { 
            mode: UiMode::Tile,
            ev_resource: events.resource_change.subscribe(),
            ev_chest: events.chest.subscribe(),
            bubbles: VecDeque::new()
        }
    }
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
    ui_state: &mut UiState,
    input_state: &InputState,
) {
    events::process_game_events(world, ui_state);
    draw_status(world, backend);
    draw_bubbles(ui_state, backend);
    input::handle_input(world, backend, input_state, ui_state);
}

fn draw_status(world: &World, backend: &dyn GraphicsBackend) {
    let Some(resources) = world.get_resource::<PlayerResources>() else { return };
    for (i, (k, v)) in resources.stock.iter().enumerate() {
        let sprite = get_resource_sprite(*k);
        span::Span::new()
            .with_sprite(sprite.0, sprite.1)
            .with_text_owned(format!("{}", v))
            .with_sprite_color(sprite.2)
            .draw(
                Vector2F::new(10., 32. + i as f32 * 32.),
                backend
            );
    }
}

fn draw_bubbles(ui_state: &mut UiState, backend: &dyn GraphicsBackend) {
    for (i, bubble) in ui_state.bubbles.iter_mut().rev().enumerate() {
        bubble.1.draw(
            Vector2F::new(10., -10. + backend.viewport_size().y - i as f32 * 32.),
            backend
        );
        bubble.0 += 10.;
    }
    if let Some(front) = ui_state.bubbles.front() {
        if front.0 > 800. { 
            ui_state.bubbles.pop_front();
        }
    }
}

fn get_resource_sprite(resource: Resource) -> (&'static str, u32, SpriteColor) {
    match resource {
        Resource::Energy => ("ascii", 4, SpriteColor(255, 128, 0, 255)),
        Resource::Food => ("ascii", 4, SpriteColor(255, 255, 0, 255))
    }
}