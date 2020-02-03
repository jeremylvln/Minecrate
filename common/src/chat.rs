use serde::{Serialize, Deserialize};

use crate::chat_color::ChatColor;

fn def_false() -> bool { false }
fn def_none<T>() -> Option<T> { None }
fn skip_false(value: &bool) -> bool { *value == false }
fn skip_none<T>(value: &Option<T>) -> bool { value.is_none() }

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatClickEvent {
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    open_url: Option<String>,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    run_command: Option<String>,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    suggest_command: Option<String>,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    change_page: Option<i32>,
}

impl ChatClickEvent {
    pub fn default() -> ChatClickEvent {
        ChatClickEvent {
            open_url: None,
            run_command: None,
            suggest_command: None,
            change_page: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHoverEvent {
    #[serde(default = "def_none")]
    #[serde(rename = "show_text")]
    #[serde(skip_serializing_if = "skip_none")]
    show_text_raw: Option<String>,

    #[serde(default = "def_none")]
    #[serde(rename = "show_text")]
    #[serde(skip_serializing_if = "skip_none")]
    show_text_chat: Option<Box<Chat>>,

    #[serde(default = "def_none")]
    #[serde(rename = "show_item")]
    #[serde(skip_serializing_if = "skip_none")]
    show_item_chat: Option<Box<Chat>>,

    // TODO
    #[serde(default = "def_none")]
    #[serde(rename = "show_item")]
    #[serde(skip_serializing_if = "skip_none")]
    show_item_nbt: Option<bool>,

    // TODO
    #[serde(default = "def_none")]
    #[serde(rename = "show_entity")]
    #[serde(skip_serializing_if = "skip_none")]
    show_entity: Option<bool>,
}

impl ChatHoverEvent {
    pub fn default() -> ChatHoverEvent {
        ChatHoverEvent {
            show_text_raw: None,
            show_text_chat: None,
            show_item_chat: None,
            show_item_nbt: None,
            show_entity: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatComponentString {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatComponentTranslation {
    translate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatComponentKeybind {
    keybind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatComponentScore {
    score: String,
    name: String,
    objective: String,
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatComponentSelector {
    selector: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    #[serde(default = "def_false")]
    #[serde(skip_serializing_if = "skip_false")]
    bold: bool,

    #[serde(default = "def_false")]
    #[serde(skip_serializing_if = "skip_false")]
    italic: bool,

    #[serde(default = "def_false")]
    #[serde(skip_serializing_if = "skip_false")]
    underlined: bool,

    #[serde(default = "def_false")]
    #[serde(skip_serializing_if = "skip_false")]
    strikethrough: bool,

    #[serde(default = "def_false")]
    #[serde(skip_serializing_if = "skip_false")]
    obfuscated: bool,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    color: Option<ChatColor>,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    insertion: Option<String>,

    #[serde(default = "def_none")]
    #[serde(rename = "clickEvent")]
    #[serde(skip_serializing_if = "skip_none")]
    click_event: Option<ChatClickEvent>,

    #[serde(default = "def_none")]
    #[serde(rename = "hoverEvent")]
    #[serde(skip_serializing_if = "skip_none")]
    hover_event: Option<ChatHoverEvent>,

    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    extra: Option<Vec<Chat>>,

    #[serde(flatten)]
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    component_string: Option<ChatComponentString>,

    #[serde(flatten)]
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    component_translation: Option<ChatComponentTranslation>,

    #[serde(flatten)]
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    component_keybind: Option<ChatComponentKeybind>,

    #[serde(flatten)]
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    component_score: Option<ChatComponentScore>,

    #[serde(flatten)]
    #[serde(default = "def_none")]
    #[serde(skip_serializing_if = "skip_none")]
    component_selector: Option<ChatComponentSelector>,
}

impl Chat {
    pub fn new_text(text: &str) -> Chat {
        Chat {
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: None,
            component_string: Some(ChatComponentString {
                text: text.to_string(),
            }),
            component_translation: None,
            component_keybind: None,
            component_score: None,
            component_selector: None,
        }
    }
}