#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_text;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::dom::{ElementData, Node, NodeType, pretty_print};
use crate::html_parser::HtmlParser;

mod dom;
mod html_parser;
mod layout;
mod render;
mod css_parser;
mod css;
mod js;

use render::render;
use css_parser::CssParser;
use crate::css::Stylesheet;

use std::sync::Mutex;

pub static mut NODES: Vec<Node> = Vec::new();

fn main() {
    // path to files
    let mut path = env::current_dir().unwrap();
    // html
    path.push("index.html");
    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };
    let mut html_input = String::new();
    file_reader.read_to_string(&mut html_input).unwrap();
    let mut parser = HtmlParser::new(&html_input);
    let nodes = parser.parse_nodes();
    // {
    //     let mut nodes_guard = NODES.lock().unwrap();
    //     *nodes_guard = nodes.clone();
    // }
    let mut body = nodes[0].children[1].clone();

    unsafe {
        NODES = nodes;
    }

    // css
    path.push("../style.css");
    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };
    let mut css_input = String::new();
    file_reader.read_to_string(&mut css_input).unwrap();
    let mut parser = CssParser::new(&css_input);
    let stylesheet = parser.parse_stylesheet();
    body.add_styles(&stylesheet);
    path.push("../index.js");
    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };
    let mut js_input = String::new();
    file_reader.read_to_string(&mut js_input).unwrap();
    body.add_js(&js_input);
    let boxes = layout::LayoutBox::build_layout_tree(&body);
    render(boxes);
}
