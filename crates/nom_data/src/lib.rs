use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;

pub struct GameData {
    pub entities: HashMap<String, EntityData>,
    pub tiles: Vec<String>
}
impl GameData {
    pub fn new() -> Self {
        GameData { 
            entities: HashMap::new(),
            tiles: Vec::new()
        }
    }
    pub fn add_entities_from_str(&mut self, s: String) -> Vec<String> {
        let mut inserted_names = Vec::new();
        let values: serde_yaml::Value = serde_yaml::from_str(&s).expect("Could not parse Yaml data");
        for (k, v) in values.as_mapping().expect("Could not parse Yaml as mapping!").into_iter() {
            let data: EntityData = serde_yaml::from_value(v.clone()).expect(
                &format!("Incorrect value for: {:?}", k)
            );
            let name = k.as_str().expect(&format!("Incorrect string key: {:?}", k));
            if self.entities.insert(name.into(), data).is_some() {
                panic!("Duplicate data at: {}", name);
            }
            inserted_names.push(name.into());
        }
        inserted_names
    }
}

#[derive(Clone, Deserialize)]
pub struct EntityData {
    pub sprite: SpriteData,
    pub spawn_chance: Option<f32>,
    pub min_distance: Option<usize>,
    pub components: serde_yaml::Value
}

#[derive(Clone, Deserialize)]
pub struct SpriteData {
    pub atlas_name: String,
    pub index: u32,
    pub color: SpriteColor
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct SpriteColor(pub u8, pub u8, pub u8, pub u8);