use std::convert::From;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
    Custom(i32),
}

impl From<Dimension> for i32 {
    fn from(value: Dimension) -> i32 {
        match value {
            Dimension::Overworld => 0,
            Dimension::Nether => -1,
            Dimension::End => 1,
            Dimension::Custom(ref x) => *x,
        }
    }
}

impl From<i32> for Dimension {
    fn from(value: i32) -> Dimension {
        match value {
            0 => Dimension::Overworld,
            -1 => Dimension::Nether,
            1 => Dimension::End,
            _ => Dimension::Custom(value),
        }
    }
}
