use std::cell::Cell;
use std::mem;

use cgmath::Vector2;
use cgmath::Vector3;

use state::rules::ResourceInfo;
use state::rules::Rules;
use state::rules::TechnologyInfo;

pub use std::vec::IntoIter as VecIntoIter;

#[derive(Clone, Debug)]
pub struct GameState;

impl GameState {
    // TODO: pass construction data as parameter instead of building it inside
    pub fn new(rules: Rules) -> GameState {
        GameState
    }
}
