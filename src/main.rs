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
mod browser;

use render::render;
use css_parser::CssParser;
use crate::css::Stylesheet;

use std::sync::Mutex;
use crate::browser::Browser;


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
    let mut browser = Browser::new(html_input);
    browser.run();
}

// 1 father element
// 2 js function or object