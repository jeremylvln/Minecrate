use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dimension {
    Overworld, Nether, End, Custom(i32)
}

impl Dimension {
    pub fn from_i32(value: &i32) -> Self {
        match value {
            0 => Dimension::Overworld,
            -1 => Dimension::Nether,
            1 => Dimension::End,
            _ => Dimension::Custom(*value)
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            &Dimension::Overworld => 0,
            &Dimension::Nether => -1,
            &Dimension::End => 1,
            &Dimension::Custom(ref x) => *x,
        }
    }
}

impl Hash for Dimension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Dimension::Overworld => state.write_i32(0),
            &Dimension::Nether => state.write_i32(-1),
            &Dimension::End => state.write_i32(1),
            &Dimension::Custom(ref x) => state.write_i32(*x),
        }
    }
}

impl Eq for Dimension {}