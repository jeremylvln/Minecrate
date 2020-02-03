use std::fmt;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

#[derive(Debug)]
pub enum ChatColor {
    Black, DarkBlue, DarkGreen, DarkCyan, DarkRed, Purple, Gold,
    Gray, DarkGray, Blue, BrightGreen, Cyan, Red, Pink, Yellow,
    White, Obfuscated, Bold, Strikethrough, Underline, Italic, Reset
}

impl ChatColor {
    pub fn from_string(value: &str) -> Option<Self> {
        match value {
            "black" => Some(ChatColor::Black),
            "dark_blue" => Some(ChatColor::DarkBlue),
            "dark_green" => Some(ChatColor::DarkGreen),
            "dark_cyan" => Some(ChatColor::DarkCyan),
            "dark_red" => Some(ChatColor::DarkRed),
            "purple" => Some(ChatColor::Purple),
            "gold" => Some(ChatColor::Gold),
            "gray" => Some(ChatColor::Gray),
            "dark_gray" => Some(ChatColor::DarkGray),
            "blue" => Some(ChatColor::Blue),
            "green" => Some(ChatColor::BrightGreen),
            "cyan" => Some(ChatColor::Cyan),
            "red" => Some(ChatColor::Red),
            "pink" => Some(ChatColor::Pink),
            "yellow" => Some(ChatColor::Yellow),
            "white" => Some(ChatColor::White),
            "obfuscated" => Some(ChatColor::Obfuscated),
            "bold" => Some(ChatColor::Bold),
            "strikethrough" => Some(ChatColor::Strikethrough),
            "underline" => Some(ChatColor::Underline),
            "italic" => Some(ChatColor::Italic),
            "reset" => Some(ChatColor::Reset),
            _ => None
        }
    }

    pub fn get_code(&self) -> char {
        match &self {
            &ChatColor::Black => '0',
            &ChatColor::DarkBlue => '1',
            &ChatColor::DarkGreen => '2',
            &ChatColor::DarkCyan => '3',
            &ChatColor::DarkRed => '4',
            &ChatColor::Purple => '5',
            &ChatColor::Gold => '6',
            &ChatColor::Gray => '7',
            &ChatColor::DarkGray => '8',
            &ChatColor::Blue => '9',
            &ChatColor::BrightGreen => 'a',
            &ChatColor::Cyan => 'b',
            &ChatColor::Red => 'c',
            &ChatColor::Pink => 'd',
            &ChatColor::Yellow => 'e',
            &ChatColor::White => 'f',
            &ChatColor::Obfuscated => 'k',
            &ChatColor::Bold => 'l',
            &ChatColor::Strikethrough => 'm',
            &ChatColor::Underline => 'n',
            &ChatColor::Italic => 'o',
            &ChatColor::Reset => 'r',
        }   
    }

    pub fn to_string(&self) -> &'static str {
        match &self {
            &ChatColor::Black => "black",
            &ChatColor::DarkBlue => "dark_blue",
            &ChatColor::DarkGreen => "dark_green",
            &ChatColor::DarkCyan => "dark_cyan",
            &ChatColor::DarkRed => "dark_red",
            &ChatColor::Purple => "purple",
            &ChatColor::Gold => "gold",
            &ChatColor::Gray => "gray",
            &ChatColor::DarkGray => "dark_gray",
            &ChatColor::Blue => "blue",
            &ChatColor::BrightGreen => "green",
            &ChatColor::Cyan => "cyan",
            &ChatColor::Red => "red",
            &ChatColor::Pink => "pink",
            &ChatColor::Yellow => "yellow",
            &ChatColor::White => "white",
            &ChatColor::Obfuscated => "obfuscated",
            &ChatColor::Bold => "bold",
            &ChatColor::Strikethrough => "strikethrough",
            &ChatColor::Underline => "underline",
            &ChatColor::Italic => "italic",
            &ChatColor::Reset => "reset",
        }
    }
}

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer {
        serializer.collect_str(self.to_string())
    }
}

impl<'de> Deserialize<'de> for ChatColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'de> {
        deserializer.deserialize_string(ChatColorVisitor)
    }
}

struct ChatColorVisitor;

impl<'de> Visitor<'de> for ChatColorVisitor {
    type Value = ChatColor;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match ChatColor::from_string(value) {
            Some(color) => Ok(color),
            None => Err(E::custom(format!("unknown color: {}", value)))
        }
    }
}
