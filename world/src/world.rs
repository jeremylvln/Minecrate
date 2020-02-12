use common::dimension::Dimension;
use common::level_type::LevelType;

use crate::chunk::Chunk;

pub struct World {
    pub dimension: Dimension,
    pub level_type: LevelType,
    pub chunk: Chunk,
}

impl World {
    pub fn new(dimension: Dimension, level_type: LevelType) -> Self {
        Self {
            dimension,
            level_type,
            chunk: Chunk {},
        }
    }
}
