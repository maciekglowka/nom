use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};

use nom_data::GameData;

use crate::GameManager;
use crate::actions::{Action, ActionQueue};
use crate::components::{Name, Position, insert_data_components};

pub fn execute_action(mut action: Box<dyn Action>, world: &mut World, manager: &GameManager) {
    let mut side_effects = Vec::new();
    let type_id = action.type_id();
    
    for modifier in manager.action_modifiers.get(&type_id).iter().flat_map(|a| *a) {
        let result = modifier(world, action);
        if result.action.type_id() != type_id {
            // the action has changed it's type
            // start over and discard potential side-effects
            world.get_resource_mut::<ActionQueue>().unwrap().0.push_front(result.action);
            return;
        }
        action = result.action;
        side_effects.extend(result.side_effects);
    }

    if action.execute(world).is_err() { return };
    let queue = &mut world.get_resource_mut::<ActionQueue>().unwrap().0;
    queue.extend(side_effects);
}

pub fn get_current_action(world: &mut World) -> Option<Box<dyn Action>> {
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