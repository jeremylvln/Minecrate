use std::collections::HashMap;
use common::dimension::Dimension;
use common::difficulty::Difficulty;
use entity::ECSWorld;
use world::world::World;

use crate::config::Config;

pub struct MinecraftServer<'a, 'b> {
    pub config: Config,
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
    pub ecs: ECSWorld<'a, 'b>,
    pub worlds: HashMap<Dimension, World>,
}

impl<'a, 'b> MinecraftServer<'a, 'b> {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            difficulty: Difficulty::Normal,
            difficulty_locked: false,
            ecs: ECSWorld::new(),
            worlds: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {

    }
}