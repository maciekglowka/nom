use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use crate::actions::{
    Action, CollectResources, EmptyAction, EnterTile, MovePlayer, ShiftBoard, TravelCost, UseResources
};
use crate::components::{Chest, Player, ResourceDemand, ResourceSupply, Tile, Position};
use crate::input::{GameInput, InputRequired};
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
    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    if let Some(resources) = world.get_resource::<PlayerResources>() {
        side_effects.push(Box::new(TravelCost { 
           value: resources.travel_cost.clone()
       }));
    }
    ActionModifierResult::new(action, side_effects)
}

pub fn shift_board_modifier(_world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    ActionModifierResult::new(action, vec![Box::new(ShiftBoard)])
}

pub fn movement_enter_tile_modifier(_world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    // dummy modifier used to trigger other actions - such as resource collection
    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    
    if let Some(move_player) = action.as_any().downcast_ref::<MovePlayer>() {
        side_effects.push(Box::new(EnterTile {
           target: move_player.target
       }));
    }
    ActionModifierResult::new(action, side_effects)
}

pub fn enter_tile_resources_modifier(world: &mut World, action: Box<dyn Action>) ->ActionModifierResult {
    let mut side_effects: Vec<Box<dyn Action>> = Vec::new();
    let Some(enter) = action.as_any().downcast_ref::<EnterTile>() else {
        return ActionModifierResult::new(action, side_effects)
    };
    let Some(entity) = get_tile_entity(world, enter.target) else {
        return ActionModifierResult::new(action, side_effects)
    };
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

pub fn enter_tile_chest_modifier(world: &mut World, action: Box<dyn Action>) -> ActionModifierResult {
    let side_effects: Vec<Box<dyn Action>> = Vec::new();
    let Some(enter) = action.as_any().downcast_ref::<EnterTile>() else {
        return ActionModifierResult::new(action, side_effects)
    };
    let Some(entity) = get_tile_entity(world, enter.target) else {
        return ActionModifierResult::new(action, side_effects)
    };
    let Some(chest) = world.get_component::<Chest>(entity) else {
        return ActionModifierResult::new(action, side_effects)
    };
    if let Some(mut input) = world.get_resource_mut::<GameInput>() {
        input.required = Some(InputRequired::Action(vec![
            ("click me".into(), Some(Box::new(EmptyAction))),
            ("empty".into(), Some(Box::new(EmptyAction))),
        ]))
    };
    ActionModifierResult::new(action, Vec::new())
}

fn get_tile_entity(world: &World, v: Vector2I) -> Option<Entity> {
    Some(
        world.query::<Tile>().with::<Position>()
            .iter()
            .find(|a| a.get::<Position>().unwrap().0 == v)?
            .entity
    )
}