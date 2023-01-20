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
            // self.consume_while(is_closing_bracket_or_letter);
            // if self.chars.peek().map_or(false, |c| *c != '}') {
            //     break;
            // }
            let property = self.parse_property();
            properties.push(property);
        }

        self.chars.next();
        rule.properties = properties;
        rule
    }

    fn parse_selector(&mut self) -> Selector {
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
        while self.chars.peek().map_or(false, |c| *c != ':') {
            name.push(self.chars.next().unwrap());
        }
        self.consume_while(is_not_property_identifier);
        let mut value = String::new();
        while self.chars.peek().map_or(false, |c| *c != ';') {
            value.push(self.chars.next().unwrap());
        }
        let (name, value) = self.check_and_process_property_members(name, value);
        property.name = name;
        property.value = value;
        property
    }

    fn check_and_process_property_members(&mut self, name: String, value: String) -> (PropertyName, PropertyValue) {
        match name.as_str() {
            "color" => (PropertyName::Color, PropertyValue::Color(self.parse_color(value))),
            _ => (PropertyName::Other, PropertyValue::Other(value)),
        }
    }

    fn parse_color(&mut self, value: String) -> Color {
        match value.clone() {
            color if color.starts_with("#") => Color::Hex(value),
            color if color.starts_with("rgb") => Color::Rgb(0, 0, 0), //todo
            color => Color::Named(value),
        }
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        match self.chars.peek() {
            Some(&c) => if is_valid_start_ident(c) {
                ident.push_str(&self.consume_while(is_valid_ident))
            },
            None => {}
        }

        ident.to_lowercase()
    }

    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string()),
        }
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
    is_closing_bracket(c) || is_letter(c)
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

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}