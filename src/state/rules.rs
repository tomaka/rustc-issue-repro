use cgmath::Vector2;

use resources;
use rustc_serialize::Decodable;


use state::map_loader;
use state::map_loader::Map;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Rules {
    pub map: Map,
    pub resources: Vec<ResourceInfo>,
    pub technologies: Vec<TechnologyInfo>,
}

#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub internal_name: String,
    pub demand_per_population: f32,
}

#[derive(Debug, Clone)]
pub struct TechnologyInfo {
    pub internal_name: String,
    pub science_points_to_unlock: u64,
    pub dependencies_names: Vec<String>,
}

pub fn load(mod_name: Option<&str>) -> Rules {
    Rules {
        map: map_loader::load(mod_name, &Vector2::new(0.0, 0.0)),
        resources: Vec::new(),
        technologies: Vec::new(),
    }
}
