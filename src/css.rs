use std::collections::HashMap;
use std::fmt;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::iter::Map;

#[derive(Debug, Default)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rule {
    pub selector: Selector,
    pub properties: HashMap<PropertyName, PropertyValue>,
}

impl Rule {
    pub fn new(name: Selector, properties: HashMap<PropertyName, PropertyValue>) -> Rule {
        Rule {
            selector: name,
            properties,
        }
    }
}



#[derive(PartialEq, Debug, Default, Clone, Eq, Hash)]
pub enum PropertyName {
    #[default]
    Color,
    BackgroundColor,
    Width,
    Height,
    Margin,
    MarginTop,
    MarginBottom,
    MarginLeft,
    MarginRight,
    Padding,
    PaddingTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    Display,
    Other,
}


#[derive(PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum DisplayType {
    #[default]
    Block,
    Inline,
    InlineBlock,
    None,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum PropertyValue {
    Color(Color),
    Length(Length),
    Display(DisplayType),
    Other(String),
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::Other("".to_string())
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Color {
    Rgb(u8, u8, u8),
    Named(String),
    Hex(u32),
}

impl Color {
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Rgb(r, g, b) => (*r, *g, *b),
            Color::Named(name) => {
                match name.as_str() {
                    "black" => (0, 0, 0),
                    "white" => (255, 255, 255),
                    "red" => (255, 0, 0),
                    "orange" => (255, 165, 0),
                    "green" => (0, 255, 0),
                    "blue" => (0, 0, 255),
                    _ => (0, 0, 0),
                }
            }
            Color::Hex(hex) => {
                let r = (hex >> 16) & 0xFF;
                let g = (hex >> 8) & 0xFF;
                let b = hex & 0xFF;
                (r as u8, g as u8, b as u8)
            }
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Hex(0xffffff)
    }
}


impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Color::Rgb(r, g, b) => write!(f, "rgb({}, {}, {})", r, g, b),
            Color::Named(name) => write!(f, "{}", name),
            Color::Hex(hex) => write!(f, "#{:06x}", hex),
        }
    }
}


#[derive(PartialEq, Debug, Default, Eq)]
pub struct Selector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Option<String>,
}

impl Selector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Option<String>) -> Selector {
        Selector {
            tag_name,
            id,
            class: classes,
        }
    }
}


#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub enum Length {
    Px(i16),
    Percent(u8),
}

impl Default for Length {
    fn default() -> Self {
        Length::Px(0)
    }
}




