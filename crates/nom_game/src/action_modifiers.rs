use rogalik::storage::World;

use crate::actions::{
    Action, CollectResources, EnterTile, MovePlayer, ShiftBoard, TravelCost, UseResources
};
use crate::components::{Player, ResourceDemand, ResourceSupply, Tile, Position};
use crate::PlayerResources;

pub struct ActionModifierResult {
    pub action: Box<dyn Action>,
    pub side_effects: Vec<Box<dyn Action>>
}
impl ActionModifierResult {
    pub fn new(action: Box<dyn Action>, side_effects: Vec<Box<dyn Action>>) -> Self {
        ActionModifierResult { action, side_effects }
    }
}

pub type ActionModifier = fn(&mut World, Box<dyn Action>) -> ActionModifierResult;

pub fn movement_cost_modifier(world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    if let Some(resources) = world.get_resource::<PlayerResources>() {
        let cost = resources.travel_cost.clone();
        return ActionModifierResult::new(action, vec![Box::new(TravelCost { value: cost })]);
    }
    ActionModifierResult::new(action, Vec::new())
}

pub fn shift_board_modifier(_world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    ActionModifierResult::new(action, vec![Box::new(ShiftBoard)])
}

pub fn movement_enter_tile_modifier(_world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    // dummy modifier used to trigger other actions - such as resource collection
    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    if let Some(movement) = action.as_any().downcast_ref::<MovePlayer>() {
        side_effects.push(Box::new(EnterTile { target: movement.target }));
    }
    ActionModifierResult::new(action, side_effects)
}

pub fn enter_tile_resources_modifier(world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    let Some(enter) = action.as_any().downcast_ref::<EnterTile>() else {
        return ActionModifierResult::new(action, Vec::new())
    };
    let entity = world.query::<Tile>().with::<Position>()
        .iter()
        .find(|a| a.get::<Position>().unwrap().0 == enter.target)
        .unwrap()
        .entity;

    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    if let Some(supply) = world.get_component::<ResourceSupply>(entity) {
        side_effects.push(
            Box::new(CollectResources {
                source: Some(entity),
                value: supply.0.clone()
            })
        )
    }
    if let Some(demand) = world.get_component::<ResourceDemand>(entity) {
        side_effects.push(
            Box::new(UseResources {
                source: Some(entity),
                value: demand.0.clone()
            })
        )
    }
    ActionModifierResult::new(action, side_effects)
}