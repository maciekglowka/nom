use rogalik::math::vectors::Vector2I;
use rogalik::storage::{Component, Entity};

pub struct Name(pub String);
impl Component for Name {}

pub struct Player;
impl Component for Player {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}