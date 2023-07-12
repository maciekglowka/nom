use rogalik::{
    math::vectors::{Vector2I, Vector2F},
    storage::World
};

use crate::{GraphicsState, GraphicsBackend};
use crate::globals::{TILE_SIZE, TILE_GAP};

pub mod sprites;
mod systems;

pub fn update(
    world: &World,
    state: &mut GraphicsState,
    backend: &dyn GraphicsBackend
) {
    systems::handle_world_events(world, state);
    sprites::update_sprites(state, world);
    sprites::draw_sprites(state, backend);
}

pub fn tile_to_world(v: Vector2I) -> Vector2F {
    // flipped y axis!
    let m = (TILE_SIZE + TILE_GAP) as f32;
    Vector2F::new(
        v.x as f32 * m,
        - v.y as f32 * m
    )
}

pub fn world_to_tile(v: Vector2F) -> Vector2I {
    let size = TILE_SIZE + TILE_GAP;
    Vector2I::new(
        (v.x / size).floor() as i32,
        (-v.y / size).ceil() as i32
    )
}

pub fn move_towards(origin: Vector2F, target: Vector2F, max_delta: f32) -> Vector2F {
    let a = target - origin;
    let l = a.len();
    if l <= max_delta || l == 0. {
        return target
    }
    origin + a / l * max_delta
} 