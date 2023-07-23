use rogalik::{
    math::vectors::Vector2I,
    storage::{Entity, World}
};

use crate::actions::{Action, ActionQueue, MovePlayer};

pub enum Input  {
    Tile(Vector2I),
    Action(Box<dyn Action>)
}

pub enum InputRequired {
    Tile,
    Action(Vec<(String, Option<Box<dyn Action>>)>)
}

#[derive(Default)]
pub struct GameInput {
    pub required: Option<InputRequired>,
    pub current: Option<Input>
}


pub fn get_current_input(world: &mut World) -> Option<Input> {
    let mut game_input = world.get_resource_mut::<GameInput>()?;
    if let Some(input) = game_input.current.take() {
        // clear the required
        let _ = game_input.required.take();
        return Some(input);
    }
    None
}

pub fn handle_input(world: &mut World, input: Input) {
    match input {
        Input::Tile(v) => {
            let Some(mut queue) = world.get_resource_mut::<ActionQueue>() else { return };
            queue.0.push_back(Box::new(MovePlayer { target: v }));
        },
        Input::Action(a) => {
            let Some(mut queue) = world.get_resource_mut::<ActionQueue>() else { return };
            queue.0.push_back(a);
        }
        _ => panic!("Not implemented!")
    }
}