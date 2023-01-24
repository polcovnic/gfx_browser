use std::fmt;
use std::default::Default;
use std::fmt::{Debug, Formatter};
use std::iter::Map;

#[derive(Debug, Default)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Default)]
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

#[derive(PartialEq, Debug, Default)]
pub struct Property {
    pub name: PropertyName,
    pub value: PropertyValue,
}


#[derive(PartialEq, Debug, Default)]
pub enum PropertyName {
    #[default]
    Color,
    BackgroundColor,
    BackgroundImage,
    Width,
    Height,
    Margin,
    Padding,
    Display,
    Border,
    BorderColor,
    BorderWidth,
    BorderStyle,
    FontSize,
    FontFamily,
    FontWeight,
    Other,
}

#[derive(PartialEq, Debug)]
pub struct Border {
    pub color: Color,
    pub width: Length,
    pub style: BorderStyle,
}

impl Border {
    pub fn with_width(&mut self, width: Length) -> &mut Self {
        self.width = width;
        self
    }
}

impl Default for Border {
    fn default() -> Self {
        Border {
            color: Color::default(),
            width: Length::default(),
            style: BorderStyle::default(),
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub enum BorderStyle {
    #[default]
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    None,
    Hidden,
}

#[derive(PartialEq, Debug, Default)]
pub enum DisplayType {
    #[default]
    Block,
    Inline,
    InlineBlock,
    None,
}

#[derive(PartialEq, Debug)]
pub enum PropertyValue {
    Color(Color),
    Length(Length),
    Border(Border),
    Display(DisplayType),
    Other(String),
}

impl Default for PropertyValue {
    fn default() -> Self {
        PropertyValue::Other("".to_string())
    }
}

#[derive(PartialEq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Named(String),
    Hex(u32),
}

impl Default for Color {
    fn default() -> Self {
        Color::Hex(0x000)
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




#[derive(PartialEq, Debug, Default)]
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


#[derive(PartialEq, Debug, Eq)]
pub enum Length {
    Px(u16),
    Percent(u8),
    Em(u16),
    Vh(u16),
}

impl Default for Length {
    fn default() -> Self {
        Length::Px(0)
    }
}




