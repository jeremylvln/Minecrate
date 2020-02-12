use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ChatColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
}

impl FromStr for ChatColor {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "black" => Ok(ChatColor::Black),
            "dark_blue" => Ok(ChatColor::DarkBlue),
            "dark_green" => Ok(ChatColor::DarkGreen),
            "dark_cyan" => Ok(ChatColor::DarkCyan),
            "dark_red" => Ok(ChatColor::DarkRed),
            "dark_purple" => Ok(ChatColor::Purple),
            "gold" => Ok(ChatColor::Gold),
            "gray" => Ok(ChatColor::Gray),
            "dark_gray" => Ok(ChatColor::DarkGray),
            "blue" => Ok(ChatColor::Blue),
            "green" => Ok(ChatColor::BrightGreen),
            "cyan" => Ok(ChatColor::Cyan),
            "red" => Ok(ChatColor::Red),
            "pink" => Ok(ChatColor::Pink),
            "yellow" => Ok(ChatColor::Yellow),
            "white" => Ok(ChatColor::White),
            "obfuscated" => Ok(ChatColor::Obfuscated),
            "bold" => Ok(ChatColor::Bold),
            "strikethrough" => Ok(ChatColor::Strikethrough),
            "underline" => Ok(ChatColor::Underline),
            "italic" => Ok(ChatColor::Italic),
            "reset" => Ok(ChatColor::Reset),
            _ => Err("Unknown ChatColor"),
        }
    }
}

impl ChatColor {
    pub fn get_code(self) -> char {
        match self {
            ChatColor::Black => '0',
            ChatColor::DarkBlue => '1',
            ChatColor::DarkGreen => '2',
            ChatColor::DarkCyan => '3',
            ChatColor::DarkRed => '4',
            ChatColor::Purple => '5',
            ChatColor::Gold => '6',
            ChatColor::Gray => '7',
            ChatColor::DarkGray => '8',
            ChatColor::Blue => '9',
            ChatColor::BrightGreen => 'a',
            ChatColor::Cyan => 'b',
            ChatColor::Red => 'c',
            ChatColor::Pink => 'd',
            ChatColor::Yellow => 'e',
            ChatColor::White => 'f',
            ChatColor::Obfuscated => 'k',
            ChatColor::Bold => 'l',
            ChatColor::Strikethrough => 'm',
            ChatColor::Underline => 'n',
            ChatColor::Italic => 'o',
            ChatColor::Reset => 'r',
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            ChatColor::Black => "black",
            ChatColor::DarkBlue => "dark_blue",
            ChatColor::DarkGreen => "dark_green",
            ChatColor::DarkCyan => "dark_cyan",
            ChatColor::DarkRed => "dark_red",
            ChatColor::Purple => "dark_purple",
            ChatColor::Gold => "gold",
            ChatColor::Gray => "gray",
            ChatColor::DarkGray => "dark_gray",
            ChatColor::Blue => "blue",
            ChatColor::BrightGreen => "green",
            ChatColor::Cyan => "cyan",
            ChatColor::Red => "red",
            ChatColor::Pink => "pink",
            ChatColor::Yellow => "yellow",
            ChatColor::White => "white",
            ChatColor::Obfuscated => "obfuscated",
            ChatColor::Bold => "bold",
            ChatColor::Strikethrough => "strikethrough",
            ChatColor::Underline => "underline",
            ChatColor::Italic => "italic",
            ChatColor::Reset => "reset",
        }
    }
}

impl Serialize for ChatColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for ChatColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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
        match ChatColor::from_str(value) {
            Ok(color) => Ok(color),
            Err(_) => Err(E::custom(format!("unknown color: {}", value))),
        }
    }
}
