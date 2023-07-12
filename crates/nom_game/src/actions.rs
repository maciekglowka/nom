use rogalik::math::vectors::Vector2I;
use rogalik::storage::World;

use crate::globals::BOARD_WIDTH;

use super::board::{Board, spawn_row};
use super::components::{Player, Position};

pub struct CurrentAction(pub Option<Box<dyn Action>>);

pub trait Action {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
}

pub struct MovePlayer {
    pub target: Vector2I
}
impl Action for MovePlayer {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        if let Some(mut position) = world.query::<Player>().with::<Position>()
            .iter().next()?.get_mut::<Position>() {
                    if self.target.y != position.0.y + 1 { return None };
                    if (self.target.x - position.0.x).abs() > 1 { return None };
                    if self.target.x < 0 || self.target.x >= BOARD_WIDTH as i32 { return None };
            
                    position.0 = self.target;
                    return Some(Box::new(ShiftBoard));
            }
        None
    }
}

pub struct ShiftBoard;
impl Action for ShiftBoard {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
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