use std::fmt;
use std::default::Default;
use std::iter::Map;

#[derive(Debug)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
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
            value: PropertyValue::Color(Color::default()),
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
    Length(u16, Unit),
    Other(String),
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Rgb(u8, u8, u8),
    Named(String),
    Hex(String),
}
impl Default for Color {
    fn default() -> Self {
        Color::Rgb(0, 0, 0)
    }
}

#[derive(PartialEq, Debug)]
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
pub enum Unit {
    Em,
    Ex,
    Vh,
    Px,
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


