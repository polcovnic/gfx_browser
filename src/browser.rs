use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::css::Stylesheet;
use crate::css_parser::CssParser;
use crate::dom::Node;
use crate::dom::NodeType::{Element, Text};
use crate::html_parser::HtmlParser;
use crate::layout;
use crate::render::render;

pub static mut NODES: Vec<Node> = Vec::new();
pub struct Browser {
    html: String,
    stylesheet: Stylesheet,
    js: String,
    nodes: Vec<Node>,
    title: String,
}

impl Browser {
    pub fn new(html: String) -> Self {
        Browser {
            html,
            stylesheet: Stylesheet::default(),
            js: String::new(),
            title: String::from("Browser"),
            nodes: Vec::new(),
        }
    }
    pub fn run(&mut self) {
        let mut parser = HtmlParser::new(self.html.as_str());
        let nodes = parser.parse_nodes();
        let head = nodes[0].children[0].clone();
        for child in head.children {
            if let Element(element_data) = &child.node_type {
                if element_data.tag_name == "title" {
                    if let Text(text) = &child.children[0].node_type {
                        self.title = text.clone();
                    }
                }
                if element_data.tag_name == "link" {
                    if let Some(rel) = element_data.attributes.get("rel") {
                        if rel == "stylesheet" {
                            if let Some(path) = element_data.attributes.get("href") {
                                self.parse_css(path);
                            }
                        }
                    }
                }
                if element_data.tag_name == "script" {
                    if !child.children.is_empty() {
                        if let Text(script) = &child.children[0].node_type {
                            if !script.is_empty() {
                                self.js = script.clone();
                            }
                        }
                    }
                    if let Some(path) = element_data.attributes.get("src") {
                        self.read_js(path);
                    }
                }
            }
        }
        let mut body = nodes[0].children[1].clone();
        body.add_styles(&self.stylesheet);
        // for js read
        let body_for_js = body.clone();
        unsafe {
            NODES = vec![body_for_js];
        }
        if !self.js.is_empty() {
            body.add_js(self.js.clone().as_str());
        }
        let boxes = layout::LayoutBox::build_layout_tree(&body);
        render(boxes, &self.title);
    }
    fn parse_css(&mut self, css_path: &str) {
        let mut path = env::current_dir().unwrap();
        path.push(css_path);
        let css = std::fs::read_to_string(path).unwrap();
        let mut parser = CssParser::new(&css);
        self.stylesheet = parser.parse_stylesheet();
    }

    fn read_js(&mut self, js_path: &String) {
        let mut path = env::current_dir().unwrap();
        path.push(js_path);
        let mut file_reader = match File::open(&path) {
            Ok(f) => BufReader::new(f),
            Err(e) => panic!("file: {}, error: {}", path.display(), e),
        };
        let mut js = String::new();
        file_reader.read_to_string(&mut js).unwrap();
        self.js += js.as_str();
    }
}