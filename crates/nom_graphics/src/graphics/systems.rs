use rogalik::storage::{World, WorldEvent};
use std::any::TypeId;

use nom_game::components::Position;

use crate::GraphicsState;
use super::sprites::get_sprite_renderer;

pub fn handle_world_events(
    world: &World,
    state: &mut GraphicsState
) {
    let mut sprites_updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentSpawned(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Position>() => {
                        state.sprites.push(
                            get_sprite_renderer(*entity, world)
                        );
                        sprites_updated = true;
                    },
                    _ => continue
                }
            },
            WorldEvent::ComponentRemoved(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Position>() => {
                        state.sprites.retain(|a| a.entity != *entity);
                    },
                    _ => continue
                }
            }
            _ => continue
        }
    }
    if sprites_updated {
        state.sort_sprites();
    }
}