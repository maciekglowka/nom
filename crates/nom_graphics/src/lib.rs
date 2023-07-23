use rogalik::{
    events::SubscriberHandle,
    math::vectors::Vector2F,
    storage::{World, WorldEvent}
};

use nom_data::SpriteColor;

pub mod globals;
pub mod graphics;
pub mod ui;

pub struct GraphicsState {
    sprites: Vec<graphics::sprites::SpriteRenderer>,
    ev_world: SubscriberHandle<WorldEvent>,
}
impl GraphicsState {
    pub fn new(world: &mut World) -> Self {
        GraphicsState { 
            sprites: Vec::new(),
            ev_world: world.events.subscribe(),
        }
    }
    pub fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }
}

pub trait GraphicsBackend {
    fn draw_world_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    );
    fn draw_ui_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    );
    fn draw_ui_text(
        &self,
        font_name: &str,
        text: &str,
        position: Vector2F,
        font_size: u32,
        color: SpriteColor
    );
    fn viewport_size(&self) -> Vector2F;
    fn text_size(
        &self,
        font_name: &str,
        text: &str,
        font_size: u32
    ) -> Vector2F;
}
