use std::collections::{HashMap, HashSet};
use std::{fmt, panic};
use crate::css::{DisplayType, Length, PropertyName, PropertyValue};
use crate::{CssParser, Stylesheet};
use crate::js;

#[derive(PartialEq, Eq, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
    pub styles: HashMap<PropertyName, PropertyValue>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
            styles: HashMap::new(),
        }
    }

    fn set_default_styles(&mut self) {
        self.styles.insert(PropertyName::Width, PropertyValue::Length(Length::Percent(100)));
        if let NodeType::Text(_s) = &self.node_type {
            self.styles.insert(PropertyName::Display, PropertyValue::Display(DisplayType::Inline));
        }
    }

    fn inherit_styles(&mut self, parent_styles: &HashMap<PropertyName, PropertyValue>) {
        for (key, value) in parent_styles {
            if key == &PropertyName::Color {
                self.styles.insert(key.clone(), value.clone());
            }
        }
    }

    fn add_styles_rec(&mut self, stylesheet: &Stylesheet, parent_styles: &HashMap<PropertyName, PropertyValue>) {
        self.set_default_styles();
        self.inherit_styles(parent_styles);
        match self.node_type {
            NodeType::Element(ref element) => {
                for rule in &stylesheet.rules {
                    if let Some(tag_name) = &rule.selector.tag_name {
                        if *tag_name == element.tag_name {
                            self.styles.extend(rule.properties.clone());
                        }
                    }
                    if let Some(id) = element.get_id() {
                        if let Some(selector_id) = &rule.selector.id {
                            if *selector_id == *id {
                                self.styles.extend(rule.properties.clone());
                            }
                        }
                    }
                    for class in element.get_classes() {
                        if let Some(selector_class) = &rule.selector.class {
                            if *selector_class == class {
                                self.styles.extend(rule.properties.clone());
                            }
                        }
                    }
                }
            },
            NodeType::Text(_) => {},
            _ => {}
        }

        for child in &mut self.children {
            child.add_styles_rec(stylesheet, &self.styles);
        }
    }

    pub fn add_styles(&mut self, stylesheet: &Stylesheet) {
        let styles = HashMap::new();
        self.add_styles_rec(stylesheet, &styles);
    }

    pub fn add_js(&mut self, js: &str) {
        let result = panic::catch_unwind(|| {
            js::init(js, &self);
        });
        if result.is_err() {
            println!("Error in JS: {}", js);
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();

        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        if attributes_string.is_empty() {
            write!(f, "{}", self.tag_name)
        } else {
            write!(f, "{} {}", self.tag_name, attributes_string)
        }
    }
}


pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => {
            println!("{}<{:?}{:?}>", indent, e, n.styles);
        },
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(child, indent_size + 2);
    }

    if let NodeType::Element(ref e) = n.node_type { println!("{}</{}>", indent, e.tag_name) }
}


#[test]
fn test_add_styles() {
    let mut class_attributes = AttrMap::new();
    let mut id_attributes = AttrMap::new();
    class_attributes.insert("class".to_string(), "orange".to_string());
    id_attributes.insert("id".to_string(), "blue".to_string());
    let mut node = Node::new(NodeType::Element(
        ElementData::new("body".to_string(), AttrMap::new())), vec![
        Node::new(NodeType::Element(
            ElementData::new("div".to_string(), class_attributes)), vec![
        ]),
        Node::new(NodeType::Element(
            ElementData::new("div".to_string(), id_attributes)), vec![
        ]),
    ]);
    let css_input = r#"
body {
    color: #772233;
    margin: 10px;
}

.orange {
    background-color: orange;
}

#blue {
    background-color: blue;
    }
    "#;
    // let mut parser = CssParser::new(&css_input);
    // let stylesheet = parser.parse_stylesheet();
    // node.add_styles(&stylesheet);
    // assert_eq!(node.styles.len(), 2);
    // assert_eq!(node.children[0].styles.len(), 1);
    // assert_eq!(node.children[1].styles.len(), 1);
}
