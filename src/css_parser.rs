use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Peekable;
use std::str::Chars;
use crate::css::*;

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}


impl<'a> CssParser<'a> {
    pub fn new(full_css: &str) -> CssParser {
        CssParser {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            let rule = self.parse_rule();
            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    fn parse_rule(&mut self) -> Rule {
        let mut rule = Rule::default();
        let mut properties = Vec::new();

        rule.selector = self.parse_selector();

        // go until meet property declaration
        self.consume_while(is_not_property_identifier);

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(is_closing_bracket_or_letter);
            if self.chars.peek().map_or(false, |c| *c == '}') {
                break;
            }
            let property = self.parse_property();
            properties.push(property);
        }

        self.chars.next();
        rule.properties = properties;
        rule
    }

    fn parse_selector(&mut self) -> Selector {
        self.consume_while(is_space);
        let mut selector = Selector::default();
        let mut name = String::new();
        let mut selector_type: u8 = 0; // 0 - tag, 1 - class. 2 - id
        if self.chars.peek().map_or(false, |c| *c == '.') {
            self.chars.next();
            selector_type = 1;
        } else if self.chars.peek().map_or(false, |c| *c == '#') {
            self.chars.next();
            selector_type = 2;
        }
        while self.chars.peek().map_or(false, |c| *c != ' ' && *c != '{' ) {
            name.push(self.chars.next().unwrap());
        }
        match selector_type {
            1 => selector.class = Some(name),
            2 => selector.id = Some(name),
            _ => selector.tag_name = Some(name),
        }
        selector
    }

    fn parse_property(&mut self) -> Property {
        let mut property = Property::default();
        let mut name = String::new();
        self.consume_while(is_space);
        while self.chars.peek().map_or(false, |c| *c != ':') {
            name.push(self.chars.next().unwrap());
        }
        self.chars.next(); // skip ':'
        self.consume_while(is_space);
        let mut value = String::new();
        while self.chars.peek().map_or(false, |c| *c != ';') {
            value.push(self.chars.next().unwrap());
        }
        let (name, value) = CssParser::process_property_members(name, value);
        property.name = name;
        property.value = value;
        property
    }

    fn process_property_members(name: String, value: String) -> (PropertyName, PropertyValue) {
        match name.as_str() {
            "color" => (PropertyName::Color, PropertyValue::Color(CssParser::parse_color(value))),
            "margin" => (PropertyName::Margin, PropertyValue::Length(CssParser::parse_length(value))),
            _ => (PropertyName::Other, PropertyValue::Other(value)),
        }
    }

    fn parse_color(value: String) -> Color {
        match value {
            color if color.starts_with('#') => Color::Hex(u32::from_str_radix(&color[1..], 16).unwrap()),
            color if color.starts_with("rgb") => {Color::Rgb(0, 0, 0)}, //todo
            color => Color::Named(color),
        }
    }

    fn parse_length(value: String) -> Length {
        let mut value = value;
        let unit = &value[value.len() - 2..];
        let value = value[..value.len() - 2].parse::<u16>().unwrap();
        match unit {
            "px" => Length::Px(value),
            "em" => Length::Em(value),
            _ => Length::Px(value),
        }
    }

    fn parse_rgb_value(&mut self, value: String, index: usize) {
        // todo parse rgb to hex
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        if let Some(&c) = self.chars.peek() { if is_valid_start_ident(c) {
            ident.push_str(&self.consume_while(is_valid_ident))
        } }

        ident.to_lowercase()
    }


    fn consume_while<F>(&mut self, condition: F) -> String
        where
            F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_closing_bracket_or_letter(c: char) -> bool {
    !(is_closing_bracket(c) || is_letter(c))
}

fn is_closing_bracket(c: char) -> bool {
    c == '}'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_not_property_identifier(c: char) -> bool {
    !is_letter(c)
}

fn is_space(c: char) -> bool {
    c == ' '
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}

#[test]
fn test_parse_stylesheet_from_file() {
    let mut path = env::current_dir().unwrap();
    path.push("style.css");
    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };
    let mut css_input = String::new();
    file_reader.read_to_string(&mut css_input).unwrap();
    let mut parser = CssParser::new(&css_input);
    let stylesheet = parser.parse_stylesheet();
    for rule in stylesheet.rules {
        println!("{:?}", rule.selector);
        for property in rule.properties {
            println!("{:?}", property);
        }
        println!();
    }
}

#[test]
fn test_parse_rule() {
    // 1 property
    let mut parser = CssParser::new("  body {   color: red; }");
    let rule = parser.parse_rule();
    assert_eq!(rule.selector.tag_name, Some("body".to_string()));
    assert_eq!(rule.properties.len(), 1);
    assert_eq!(rule.properties[0].name, PropertyName::Color);
    assert_eq!(rule.properties[0].value, PropertyValue::Color(Color::Named("red".to_string())));
    // 2 properties
    let mut parser = CssParser::new("  body {   color: red;  \n margin: 10px;  }");
    let rule = parser.parse_rule();
    assert_eq!(rule.selector.tag_name, Some("body".to_string()));
    assert_eq!(rule.properties.len(), 2);
    assert_eq!(rule.properties[1].name, PropertyName::Margin);
    assert_eq!(rule.properties[1].value, PropertyValue::Length(Length::Px(10)));
}
#[test]
fn test_parse_selector() {
    // test tag name
    let mut parser = CssParser::new("  body  { color: red; }");
    let selector = parser.parse_selector();
    assert_eq!(selector.tag_name, Some("body".to_string()));
    assert_eq!(selector.id, None);
    assert_eq!(selector.class, None);
    // test id
    let mut parser = CssParser::new("  #id    { color: red; }");
    let selector = parser.parse_selector();
    assert_eq!(selector.tag_name, None);
    assert_eq!(selector.id, Some("id".to_string()));
    assert_eq!(selector.class, None);
    // test class
    let mut parser = CssParser::new("  .class   { color: red; }");
    let selector = parser.parse_selector();
    assert_eq!(selector.tag_name, None);
    assert_eq!(selector.id, None);
    assert_eq!(selector.class, Some("class".to_string()));
}

#[test]
fn test_parse_property() {
    // test named color
    let mut parser = CssParser::new("  color: red; }");
    let property = parser.parse_property();
    assert_eq!(property.name, PropertyName::Color);
    assert_eq!(property.value, PropertyValue::Color(Color::Named("red".to_string())));
}

#[test]
fn test_process_property_members_color() {
    let color_key = String::from("color");
    let color_value = String::from("red");
    let (name, value) =
        CssParser::process_property_members(color_key, color_value);
    assert_eq!(name, PropertyName::Color);
    assert_eq!(value, PropertyValue::Color(Color::Named("red".to_string())));
}

#[test]
fn test_process_property_members_margin() {
   let margin_key = String::from("margin");
    let margin_value = String::from("10px");
    let (name, value) =
        CssParser::process_property_members(margin_key, margin_value);
    assert_eq!(name, PropertyName::Margin);
    assert_eq!(value, PropertyValue::Length(Length::Px(10)));
}

#[test]
fn test_parse_color() {
    // test named color
    let value = String::from("red");
    let color = CssParser::parse_color(value);
    assert_eq!(color, Color::Named("red".to_string()));
    // test hex color
    let value = String::from("#ff0000");
    let color = CssParser::parse_color(value);
    assert_eq!(color, Color::Hex(0xff0000));
    // test rgb color
    //todo
}
