use std::collections::HashMap;
use common::dimension::Dimension;
use common::difficulty::Difficulty;
use world::world::World;

use crate::config::Config;

pub struct MinecraftServer {
    pub config: Config,
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
    pub worlds: HashMap<Dimension, World>,
}

impl MinecraftServer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            difficulty: Difficulty::Normal,
            difficulty_locked: false,
            worlds: HashMap::new(),
        }
    }
}