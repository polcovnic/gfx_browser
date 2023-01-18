#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_text;

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::dom::pretty_print;
use crate::html_parser::HtmlParser;

mod dom;
mod html_parser;
mod layout;
mod render;
use render::render;


fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("index.html");
    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };
    let mut html_input = String::new();
    file_reader.read_to_string(&mut html_input).unwrap();
    let mut parser = HtmlParser::new(&html_input);
    let nodes = parser.parse_nodes();
    pretty_print(&nodes[0], 0);
    let boxes = layout::build_layout_tree(&nodes[0]);
    println!("{:?}", boxes);
    render(boxes);
}
