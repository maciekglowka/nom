use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque}
};

use crate::globals::BOARD_WIDTH;

use super::board::{Board, spawn_row};
use super::components::{Player, Position, Tile, ResourceSupply, ResourceDemand};
use super::resources::{PlayerResources, Resource};

pub struct ActionQueue(pub VecDeque<Box<dyn Action>>);

pub trait Action {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>>;
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

pub struct MovePlayer {
    pub target: Vector2I
}
impl Action for MovePlayer {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        if let Some(mut position) = world.query::<Player>().with::<Position>()
            .iter().next()?.get_mut::<Position>() {
                    if self.target.y != position.0.y + 1 { return None };
                    if (self.target.x - position.0.x).abs() > 1 { return None };
                    if self.target.x < 0 || self.target.x >= BOARD_WIDTH as i32 { return None };

                    let cost = world.get_resource::<PlayerResources>()?.travel_cost.clone();
            
                    position.0 = self.target;
                    return Some(vec![
                        Box::new(ShiftBoard),
                        Box::new(TravelCost { value: cost }),
                        Box::new(EnterTile { target: self.target })
                    ]);
            }
        None
    }
}

pub struct EnterTile {
    pub target: Vector2I
}
impl Action for EnterTile {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let entity = world.query::<Tile>().with::<Position>()
            .iter()
            .find(|a| a.get::<Position>().unwrap().0 == self.target)?
            .entity;
        let mut result: Vec<Box<dyn Action>> = Vec::new();
        if let Some(supply) = world.get_component::<ResourceSupply>(entity) {
            result.push(
                Box::new(CollectResources {
                    source: Some(entity),
                    value: supply.0.clone()
                })
            )
        }
        if let Some(demand) = world.get_component::<ResourceDemand>(entity) {
            result.push(
                Box::new(UseResources {
                    source: Some(entity),
                    value: demand.0.clone()
                })
            )
        }
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }
}

pub struct TravelCost {
    pub value: HashMap<Resource, i32>
}
impl Action for TravelCost {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut resources = world.get_resource_mut::<PlayerResources>()?;
        resources.remove_resources(&self.value);
        None
    }
}

pub struct CollectResources {
    pub source: Option<Entity>,
    pub value: HashMap<Resource, i32>
}
impl Action for CollectResources {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut resources = world.get_resource_mut::<PlayerResources>()?;
        resources.add_resources(&self.value);
        // println!("Collected: {:?}", self.value);
        None
    }
}

pub struct UseResources {
    pub source: Option<Entity>,
    pub value: HashMap<Resource, i32>
}
impl Action for UseResources {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut resources = world.get_resource_mut::<PlayerResources>()?;
        resources.remove_resources(&self.value);
        None
    }
}

pub struct ShiftBoard;
impl Action for ShiftBoard {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let to_remove = world.get_resource::<Board>()?
            .tiles
            .get(0)?
            .iter()
            .map(|&e| e)
            .collect::<Vec<_>>();
        
        for entity in to_remove {
            world.despawn_entity(entity);
        }

        spawn_row(world);
        if let Some(mut board) = world.get_resource_mut::<Board>() {
            board.tiles.pop_front();
        }
        None
    }
}