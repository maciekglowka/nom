use macroquad::prelude::*;
use rogalik::math::vectors::{Vector2I, Vector2F};
use rogalik::storage::World;

use nom_game::actions::{CurrentAction, MovePlayer};
use nom_graphics::graphics::world_to_tile;

fn get_mouse_world_position(camera: &Camera2D) -> Vector2F {
    let mouse = mouse_position();
    let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
    Vector2F::new(v.x, v.y)
}

fn get_mouse_tile(camera: &Camera2D) -> Vector2I {
    world_to_tile(get_mouse_world_position(camera))
}

pub fn set_input_action(camera: &Camera2D, world: &mut World) {
    if is_mouse_button_released(MouseButton::Left) {
        let Some(mut current) = world.get_resource_mut::<CurrentAction>() else { return };
        if current.0.is_some() { return };
        current.0 = Some(Box::new(MovePlayer { target: get_mouse_tile(camera) }));
    }
}