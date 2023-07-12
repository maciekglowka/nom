use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Component, Entity, World};
use serde::Deserialize;
use serde_yaml;

// Dynamically inserted components
pub struct Position(pub Vector2I);
impl Component for Position {}

// Deserialized components
pub struct Name(pub String);
impl Component for Name {}

#[derive(Deserialize)]
pub struct Player;
impl Component for Player {}

#[derive(Deserialize)]
pub struct Tile;
impl Component for Tile {}

pub fn insert_data_components(
    entity: Entity,
    world: &mut World,
    value: &serde_yaml::Value
) {
    let Some(data) = value.as_mapping() else { return };
    for (name, component_data) in data.iter() {
        let Some(name) = name.as_str() else { continue };
        match name {
            "Player" => insert_single::<Player>(entity, world, component_data),
            "Tile" => insert_single::<Tile>(entity, world, component_data),
            _ => continue
        };
    }
}

fn insert_single<T>(
    entity: Entity,
    world: &mut World,
    data: &serde_yaml::Value
) where for<'de> T: 'static + Component + Deserialize<'de> {
    let Ok(component) = serde_yaml::from_value::<T>(data.clone()) else { return };
    let _ =world.insert_component(entity, component);
}