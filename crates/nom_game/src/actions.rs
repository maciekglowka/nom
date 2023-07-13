use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;
use std::collections::{HashMap, VecDeque};

use crate::globals::BOARD_WIDTH;

use super::board::{Board, spawn_row};
use super::components::{Player, Position, Tile, Resources};
use super::resources::{PlayerResources, Resource};

pub struct ActionQueue(pub VecDeque<Box<dyn Action>>);

pub trait Action {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>>;
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
                    return Some(vec![Box::new(ShiftBoard), Box::new(HandleResources)]);
            }
        None
    }
}

pub struct HandleResources;
impl Action for HandleResources {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut resources = world.get_resource_mut::<PlayerResources>()?;

        // base movement cost
        resources.change_stock_by(&HashMap::from_iter([
            (Resource::Food, -5), (Resource::Energy, -3)
        ]));

        // tile resources
        let position = world.query::<Player>().with::<Position>()
            .iter().next()?.get::<Position>()?.0;
        let tile_query = world.query::<Tile>()
            .with::<Position>()
            .with::<Resources>();
        let tile = tile_query.iter()
            .find(|a| a.get::<Position>().unwrap().0 == position)?;
        resources.change_stock_by(&tile.get::<Resources>()?.0);    
            
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