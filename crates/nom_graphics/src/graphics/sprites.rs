use rogalik::{
    math::vectors::Vector2F,
    storage::{Entity, World}
};

use nom_data::{GameData, SpriteColor, SpriteData};
use nom_game::components::{Name, Player, Position, Tile};

use crate::{GraphicsState, GraphicsBackend};
use crate::globals::{TILE_SIZE, TILE_Z, MOVEMENT_SPEED, PLAYER_Z};
use super::{move_towards, tile_to_world};

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: u32,
    pub color: SpriteColor
}

pub fn update_sprites(state: &mut GraphicsState, world: &World) {
    for sprite in state.sprites.iter_mut() {
        let entity = sprite.entity;
        let Some(target) = world.get_component::<Position>(entity) else { continue };
        let v = tile_to_world(target.0);
        sprite.v = move_towards(sprite.v, v, MOVEMENT_SPEED);
    }
}

pub fn draw_sprites(state: &GraphicsState, backend: &dyn GraphicsBackend) {
    for sprite in state.sprites.iter() {
        backend.draw_world_sprite(
            &sprite.atlas_name,
            sprite.index,
            sprite.v,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            sprite.color
        );
    }
}

pub fn get_sprite_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let mut z_index = 0;

    let name = world.get_component::<Name>(entity).unwrap();
    let position = world.get_component::<Position>(entity).unwrap();
    let data = get_sprite_data(&name.0, world);

    if world.get_component::<Tile>(entity).is_some() {
        z_index = TILE_Z
    }
    if world.get_component::<Player>(entity).is_some() {
        z_index = PLAYER_Z
    }

    SpriteRenderer { 
        entity: entity,
        v: tile_to_world(position.0),
        atlas_name: data.atlas_name,
        index: data.index,
        z_index,
        color: data.color
    }
}

fn get_sprite_data(name: &str, world: &World) -> SpriteData {
    let game_data = world.get_resource::<GameData>().expect("Invalid game data resource!");
    game_data.entities.get(name).expect(&format!("No sprite data for {}", name)).sprite.clone()
}