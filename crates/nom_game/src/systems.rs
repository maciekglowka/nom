use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};

use nom_data::GameData;

use crate::actions::{Action, ActionQueue};
use crate::components::{Name, Position, insert_data_components};

pub fn execute_action(world: &mut World) {
    let Some(action) = get_current_action(world) else { return };
    let next = action.execute(world);
    if let Some(next) = next {
        world.get_resource_mut::<ActionQueue>().unwrap().0.extend(next);
    }
}

fn get_current_action(world: &mut World) -> Option<Box<dyn Action>> {
    let mut queue = world.get_resource_mut::<ActionQueue>()?;
    queue.0.pop_front()
}

pub fn spawn_with_position(
    world: &mut World,
    name: &str,
    position: Vector2I
) -> Option<Entity> {
    let entity = world.spawn_entity();
    let _ = world.insert_component(entity, Name(name.into()));
    let _ = world.insert_component(entity, Position(position));

    let tile_data = world.get_resource::<GameData>()?
        .entities.get(name).expect(&format!("Could not spawn: {} - no data found!", name)).clone();
    insert_data_components(entity, world, &tile_data.components);
    Some(entity)
}