use rogalik::math::vectors::Vector2F;
use rogalik::storage::World;

use nom_data::SpriteColor;
use nom_game::{PlayerResources, Resource};

use super::GraphicsBackend;

pub fn ui_update(
    world: &World,
    backend: &dyn GraphicsBackend
) {
    draw_status(world, backend);
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