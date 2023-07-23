use rogalik::{
    events::EventBus,
    storage::Entity
};

use crate::resources::Resource;

pub struct Events {
    pub resource_change: EventBus<ResourceChangeEvent>,
    pub chest: EventBus<ChestEvent>,
}
impl Events {
    pub fn new() -> Self {
        Events { 
            resource_change: EventBus::new(),
            chest: EventBus::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ResourceChangeEvent(pub Resource, pub i32);

#[derive(Clone, Copy)]
pub struct ChestEvent(pub Entity);