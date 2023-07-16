use rogalik::storage::World;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Deserialize)]
pub enum Resource {
    Food,
    Energy
}
impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            Resource::Food => "F",
            Resource::Energy => "E",
        };
        write!(f, "{}", t)
    }
}

pub struct PlayerResources {
    pub stock: HashMap<Resource, i32>,
    pub travel_cost: HashMap<Resource, i32>
}
impl PlayerResources {
    pub fn new() -> Self {
        PlayerResources { 
            stock: HashMap::new(),
            travel_cost: HashMap::from_iter([
                (Resource::Food, 4), (Resource::Energy, 2)
            ])
        }
    }
    pub fn add_resources(&mut self, value: &HashMap<Resource, i32>) {
        for (k, v) in value.iter() {
            *self.stock.entry(*k).or_insert(0) += v;
        }
    }
    pub fn remove_resources(&mut self, value: &HashMap<Resource, i32>) {
        for (k, v) in value.iter() {
            *self.stock.entry(*k).or_insert(0) -= v;
        }
    }
}