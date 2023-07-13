use rogalik::storage::World;
use std::{
    collections::HashMap,
    fmt
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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
    pub stock: HashMap<Resource, u32>
}
impl PlayerResources {
    pub fn new() -> Self {
        PlayerResources { stock: HashMap::new() }
    }
    pub fn add(&mut self, value: HashMap<Resource, u32>) {
        for (k, v) in value.iter() {
            *self.stock.entry(*k).or_insert(0) += v;
        }
    }
    pub fn remove(&mut self, value: HashMap<Resource, u32>) {
        for (k, v) in value.iter() {
            let item = self.stock.entry(*k).or_insert(0);
            *item = item.saturating_sub(*v);
        }
    }
}