use rogalik::storage::World;

use crate::actions::Action;

pub struct ActionHandlerResult {
    pub action: Box<dyn Action>,
    pub side_effects: Vec<Box<dyn Action>>
}
impl ActionHandlerResult {
    pub fn new(action: Box<dyn Action>, side_effects: Vec<Box<dyn Action>>) -> Self {
        ActionHandlerResult { action, side_effects }
    }
}

pub type ActionHandler = fn(&World, Box<dyn Action>) -> ActionHandlerResult;

// pub fn dummy_shift_handler(world: &World, action: Box<dyn Action>) -> ActionHandlerResult {
//     ActionHandlerResult::new(action, Vec::new())
// }

// pub fn dummy_move_handler(world: &mut World, action: Box<dyn Action>) -> Box<dyn Action> {

// }