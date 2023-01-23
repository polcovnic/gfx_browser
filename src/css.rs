use std::fmt;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::iter::Map;

#[derive(Debug, Clone)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub selector: Selector,
    pub properties: Vec<Property>,
}

impl Rule {
    pub fn new(name: Selector, properties: Vec<Property>) -> Rule {
        Rule {
            selector: name,
            properties,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Property {
    pub name: PropertyName,
    pub value: PropertyValue,
}

impl Default for Property {
    fn default() -> Self {
        Property {
            name: PropertyName::Color,
            value: PropertyValue::Other("".to_string()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PropertyName {
    Color,
    Margin,
    Padding,
    Other,
}

#[derive(PartialEq, Debug)]
pub enum PropertyValue {
    Color(Color),
    Length(Length),
    Other(String),
}

#[derive(PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Named(String),
    Hex(u32),
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
impl Default for Color {
    fn default() -> Self {
        Color::Rgb(0, 0, 0)
    }
}


#[derive(PartialEq, Debug, Clone)]
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

impl Default for Selector {
    fn default() -> Self {
        Selector {
            tag_name: None,
            id: None,
            class: None,
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
pub enum Length {
    Em(u16),
    Vh(u16),
    Px(u16),
}

impl Default for Stylesheet {
    fn default() -> Stylesheet {
        Stylesheet { rules: Vec::new() }
    }
}

impl Default for Rule {
    fn default() -> Rule {
        Rule {
            selector: Selector::default(),
            properties: Vec::new(),
        }
    }
}


