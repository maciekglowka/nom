use rogalik::storage::World;

use crate::actions::Action;

pub struct ActionModifierResult {
    pub action: Box<dyn Action>,
    pub side_effects: Vec<Box<dyn Action>>
}
impl ActionModifierResult {
    pub fn new(action: Box<dyn Action>, side_effects: Vec<Box<dyn Action>>) -> Self {
        ActionModifierResult { action, side_effects }
    }
}

pub type ActionModifier = fn(&World, Box<dyn Action>) -> ActionModifierResult;

// pub fn dummy_shift_handler(world: &World, action: Box<dyn Action>) -> ActionModifierResult {
//     ActionModifierResult::new(action, Vec::new())
// }

// pub fn dummy_move_handler(world: &mut World, action: Box<dyn Action>) -> Box<dyn Action> {

// }