use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Entity, World};
use std::{
    any::{Any, TypeId},
    collections::{HashMap, VecDeque}
};

use crate::globals::BOARD_WIDTH;

use super::board::{Board, spawn_row};
use super::components::{Player, Position};
use super::events;
use super::resources::{PlayerResources, Resource};

pub struct ActionQueue(pub VecDeque<Box<dyn Action>>);

pub trait Action {
    fn as_any(&self) -> &dyn Any;
    fn execute(&self, world: &mut World) -> Result<(), ()>;
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

pub struct EmptyAction;
impl Action for EmptyAction {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> { Ok(()) }
}

pub struct MovePlayer {
    pub target: Vector2I
}
impl Action for MovePlayer {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        if let Some(mut position) = world.query::<Player>().with::<Position>()
            .iter().next().ok_or(())?.get_mut::<Position>() {
                    if self.target.y != position.0.y + 1 { return Err(()) };
                    if (self.target.x - position.0.x).abs() > 1 { return Err(()) };
                    if self.target.x < 0 || self.target.x >= BOARD_WIDTH as i32 { return Err(()) };
            
                    position.0 = self.target;
                    return Ok(())
            }
        Err(())
    }
}

pub struct EnterTile {
    pub target: Vector2I
}
impl Action for EnterTile {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        Ok(())
    }
}

pub struct TravelCost {
    pub value: HashMap<Resource, i32>
}
impl Action for TravelCost {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        let mut resources = world.get_resource_mut::<PlayerResources>().ok_or(())?;
        resources.remove_resources(&self.value);
        send_resource_events(world, &self.value, true);
        Ok(())
    }
}

pub struct CollectResources {
    pub source: Option<Entity>,
    pub value: HashMap<Resource, i32>
}
impl Action for CollectResources {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        let mut resources = world.get_resource_mut::<PlayerResources>().ok_or(())?;
        resources.add_resources(&self.value);
        send_resource_events(world, &self.value, false);
        Ok(())
    }
}

pub struct UseResources {
    pub source: Option<Entity>,
    pub value: HashMap<Resource, i32>
}
impl Action for UseResources {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        let mut resources = world.get_resource_mut::<PlayerResources>().ok_or(())?;
        resources.remove_resources(&self.value);
        send_resource_events(world, &self.value, true);
        Ok(())
    }
}

fn send_resource_events(world: &World, value: &HashMap<Resource, i32>, negative: bool) {
    let m = if negative { -1 } else { 1 };
    if let Some(mut ev) = world.get_resource_mut::<events::Events>() {
        for (k, v) in value.iter() {
            ev.resource_change.publish(events::ResourceChangeEvent(*k, m * v));
        } 
    }
}

pub struct ShiftBoard;
impl Action for ShiftBoard {
    fn as_any(&self) -> &dyn Any { self }
    fn execute(&self, world: &mut World) -> Result<(), ()> {
        let to_remove = world.get_resource::<Board>().ok_or(())?
            .tiles
            .get(0).ok_or(())?
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
        Ok(())
    }
}