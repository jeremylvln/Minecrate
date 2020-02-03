#[derive(Debug)]
pub enum Dimension {
    Overworld, Nether, End
}

impl Dimension {
    pub fn from_u8(value: &i32) -> Option<Self> {
        match value {
            0 => Some(Dimension::Overworld),
            -1 => Some(Dimension::Nether),
            1 => Some(Dimension::End),
            _ => None
        }
    }

    pub fn to_u8(&self) -> i32 {
        match self {
            &Dimension::Overworld => 0,
            &Dimension::Nether => -1,
            &Dimension::End => 1,
        }
    }
}