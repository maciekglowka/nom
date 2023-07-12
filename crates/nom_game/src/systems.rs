use rogalik::storage::World;

use super::actions::{Action, CurrentAction};

pub fn execute_action(world: &mut World) {
    let Some(action) = get_current_action(world) else { return };
    let next = action.execute(world);
    world.get_resource_mut::<CurrentAction>().unwrap().0 = next;
}

fn get_current_action(world: &mut World) -> Option<Box<dyn Action>> {
    let mut current = world.get_resource_mut::<CurrentAction>()?;
    current.0.take()
}