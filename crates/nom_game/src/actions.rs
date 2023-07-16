use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;
use std::{
    any::TypeId,
    collections::{HashMap, VecDeque}
};

use crate::globals::BOARD_WIDTH;

use super::board::{Board, spawn_row};
use super::components::{Player, Position, Tile, Resources};
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
            
                    position.0 = self.target;
                    return Some(vec![
                        Box::new(ShiftBoard),
                        Box::new(TravelCost { resource_change: HashMap::from_iter([
                                (Resource::Food, -5), (Resource::Energy, -3)
                        ])})
                    ]);
            }
        None
    }
}

pub struct TravelCost {
    pub resource_change: HashMap<Resource, i32>
}
impl Action for TravelCost {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut resources = world.get_resource_mut::<PlayerResources>()?;
        resources.change_stock_by(&self.resource_change);
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