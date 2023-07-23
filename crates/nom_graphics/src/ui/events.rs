use rogalik::storage::World;
use std::collections::HashMap;

use nom_game::{
    actions,
    components,
    events
};

use super::{UiState, UiMode, get_resource_sprite, SpriteColor};
use super::modal::ModalData;
use super::span::Span;

pub fn process_game_events(
    world: &mut World,
    ui_state: &mut UiState,
) {
    for ev in ui_state.ev_resource.read().iter().flatten() {
        handle_resource_event(*ev, ui_state);
    }
    for ev in ui_state.ev_chest.read().iter().flatten() {
        handle_chest_event(world, *ev, ui_state);
    }
}

fn handle_resource_event(
    event: events::ResourceChangeEvent,
    ui_state: &mut UiState
) {
    let sprite = get_resource_sprite(event.0);
    let prefix = if event.1 > 0 { "+" } else { "" };
    ui_state.bubbles.push_back((
        0.,
        Span::new()
            .with_sprite(sprite.0, sprite.1)
            .with_sprite_color(sprite.2)
            .with_text_owned(format!("{}{}", prefix, event.1))
    ));
}

fn handle_chest_event(
    world: &mut World,
    event: events::ChestEvent,
    ui_state: &mut UiState,
) {
    let Some(chest) = world.get_component::<components::Chest>(event.0) else { return };

    let choices = chest.options.iter()
        .map(|a| {
            let sprite = get_resource_sprite(a.0);
            (
                Span::new()
                    .with_sprite(sprite.0, sprite.1)
                    .with_sprite_color(sprite.2)
                    .with_text_owned(format!("{}", a.1)),
                Some(Box::new(actions::CollectResources { 
                    source: None,
                    value: HashMap::from_iter([(a.0, a.1)]) 
                }) as Box<dyn actions::Action>)
            )
        })
        .collect::<Vec<_>>();

    ui_state.mode = UiMode::Modal(
        ModalData {
            text: "Please select:".into(),
            choices
        }
    );
}