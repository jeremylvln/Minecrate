#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LevelType {
    Default, Flat, LargeBiomes, Amplified, Customized, Buffet, Default1_1
}

impl LevelType {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "default" => Some(LevelType::Default),
            "flat" => Some(LevelType::Flat),
            "largeBiomes" => Some(LevelType::LargeBiomes),
            "amplified" => Some(LevelType::Amplified),
            "customized" => Some(LevelType::Customized),
            "buffet" => Some(LevelType::Buffet),
            "default_1_1" => Some(LevelType::Default1_1),
            _ => None
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            &LevelType::Default => "default",
            &LevelType::Flat => "flat",
            &LevelType::LargeBiomes => "largeBiomes",
            &LevelType::Amplified => "amplified",
            &LevelType::Customized => "customized",
            &LevelType::Buffet => "buffet",
            &LevelType::Default1_1 => "default_1_1",
        }
    }
}

impl Eq for LevelType {}