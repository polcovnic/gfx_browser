use std::collections::hash_set::Union;
use std::fmt;
use crate::css::{Length, PropertyName, PropertyValue};
use crate::{css, dom, layout};
use crate::css_parser::CssParser;
use crate::dom::{ElementData, NodeType};
use crate::html_parser::HtmlParser;
use crate::render::render;

#[derive(Clone, PartialEq)]
pub struct LayoutBox {
    pub dimensions: Dimensions,
    pub actual_dimensions: Dimensions,
    // for rendering
    pub content: Option<Content>,
    pub color: Color,
    pub background_color: Color,
    pub name: String,
    pub margin: Indentations,
    pub padding: Indentations,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox>,
    v_elements: i16,
    h_elements: i16,
}


#[derive(Clone, Default, PartialEq, Debug)]
pub struct Content {
    pub x: i16,
    pub y: i16,
    pub text: String,
}


#[derive(Clone, Default, PartialEq, Debug)]
pub struct Dimensions {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}

impl fmt::Debug for LayoutBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {{", self.name)?;
        for child in &self.children {
            write!(f, "{:?}", child)?;
        }
        write!(f, "}}")
    }
}


#[derive(Clone, PartialEq)]
pub enum BoxType {
    Block,
    Inline,
}


#[derive(Clone, Default, PartialEq)]
pub struct Indentations {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}


#[derive(Clone, Default, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }


    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

impl Default for LayoutBox {
    fn default() -> LayoutBox {
        LayoutBox {
            content: None,
            dimensions: Dimensions::default(),
            actual_dimensions: Dimensions::default(),
            color: Color::new(0, 0, 0, 255),
            margin: Indentations::default(),
            background_color: Color::new(255, 255, 255, 255),
            name: String::from("default"),
            box_type: BoxType::Block,
            padding: Indentations::default(),
            children: vec![],
            v_elements: 0,
            h_elements: 0,
        }
    }
}

impl LayoutBox {
    pub fn build_layout_tree(node: &dom::Node) -> Vec<LayoutBox> {
        let mut base = LayoutBox::default();
        base.dimensions.width = crate::render::WIDTH as i16;
        base.actual_dimensions.width = crate::render::WIDTH as i16;
        let mut body = LayoutBox::build_box(node, &mut base,
                                            &ElementData::default(), 0);
        body.name = if let NodeType::Element(element_data) = &node.node_type {
            element_data.tag_name.clone()
        } else {
            panic!("Root node must be an element");
        };
        body.children = LayoutBox::build_layout_tree_helper(&node.children, &mut body, 0);
        body.expand_blocks_that_have_text();
        vec![body]
    }

    pub fn expand_blocks_that_have_text(&mut self) {
        LayoutBox::expand_blocks_that_have_text_rec(self);
    }

    fn expand_blocks_that_have_text_rec(parent: &mut LayoutBox) {
        let mut count: i16 = 0;
        for i in 0..parent.children.len() {
            if parent.children[i].content.is_some() {
                parent.children[i].content.as_mut().unwrap().y += 20 * count;
                count += 1;
            }
            if i < parent.children.len() - 1 {
                parent.children[i + 1].actual_dimensions.y += 20 * count;
            }
            parent.actual_dimensions.height += parent.children[i].actual_dimensions.height;
            LayoutBox::expand_blocks_that_have_text_rec(&mut parent.children[i]);
        }
    }


    fn build_layout_tree_helper(nodes: &Vec<dom::Node>, parent: &mut LayoutBox, element_number: usize) -> Vec<LayoutBox> {
        let mut boxes = Vec::new();
        for (i, node) in nodes.iter().enumerate() {
            match &node.node_type {
                NodeType::Element(element) => {
                    let mut box_ = LayoutBox::build_box(node, parent, element, i);

                    box_.children = LayoutBox::build_layout_tree_helper(&node.children, &mut box_, element_number + 1);
                    boxes.push(box_);
                }
                NodeType::Text(text) => {
                    parent.set_content(text);
                }
                _ => {}
            }
        }
        LayoutBox::expand_parent_elements(&mut boxes);
        boxes
    }

    fn build_box(element: &dom::Node, parent: &mut LayoutBox, element_data: &ElementData, element_number: usize) -> LayoutBox {
        let mut box_ = LayoutBox::default();
        for (name, value) in &element.styles {
            match name {
                PropertyName::Color => {
                    if let PropertyValue::Color(color) = &value {
                        let color = color.get_rgb();
                        box_.color = Color::new(color.0, color.1, color.2, 255);
                    }
                }
                PropertyName::BackgroundColor => {
                    if let PropertyValue::Color(color) = &value {
                        let color = color.get_rgb();
                        box_.background_color = Color::new(color.0, color.1, color.2, 255);
                    }
                }
                PropertyName::Margin => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.margin = Indentations {
                            top: *px,
                            right: *px,
                            bottom: *px,
                            left: *px,
                        };
                    }
                }
                PropertyName::MarginTop => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.margin.top = *px;
                    }
                }
                PropertyName::MarginRight => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.margin.right = *px;
                    }
                }
                PropertyName::MarginBottom => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.margin.bottom = *px;
                    }
                }
                PropertyName::MarginLeft => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.margin.left = *px;
                    }
                }
                PropertyName::Padding => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.padding = Indentations {
                            top: *px,
                            right: *px,
                            bottom: *px,
                            left: *px,
                        };
                    }
                }
                PropertyName::PaddingTop => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.padding.top = *px;
                    }
                }
                PropertyName::PaddingRight => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.padding.right = *px;
                    }
                }
                PropertyName::PaddingBottom => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.padding.bottom = *px;
                    }
                }
                PropertyName::PaddingLeft => {
                    if let PropertyValue::Length(Length::Px(px)) = &value {
                        box_.padding.left = *px;
                    }
                }
                PropertyName::Display => {
                    if let PropertyValue::Display(display) = &value {
                        box_.box_type = match display {
                            css::DisplayType::Block => BoxType::Block,
                            css::DisplayType::Inline => BoxType::Inline,
                            _ => BoxType::Block,
                        };
                    }
                }
                PropertyName::Width => {
                    match &value {
                        PropertyValue::Length(Length::Px(px)) => {
                            box_.dimensions.width = *px;
                        }
                        PropertyValue::Length(Length::Percent(percent)) => {
                            box_.dimensions.width = parent.actual_dimensions.width * (*percent as i16 / 100);
                        }
                        _ => { panic!("Width must be a length") }
                    }
                }
                PropertyName::Height => {
                    match &value {
                        PropertyValue::Length(Length::Px(px)) => {
                            box_.dimensions.height = *px;
                        }
                        PropertyValue::Length(Length::Percent(percent)) => {
                            box_.dimensions.height = parent.dimensions.height * *percent as i16;
                        }
                        _ => { panic!("Height must be a length") }
                    }
                }
                _s => { println!("kurwa{:?}", value) }
            }
        }
        box_.name = element_data.tag_name.clone();
        box_.calculate_position(parent, element_number);
        box_.calculate_actual_dimensions(parent);
        box_
    }

    fn set_content(&mut self, str: &String) {
        let mut content = Content::default();
        content.x = self.actual_dimensions.x + self.padding.left;
        content.y = self.actual_dimensions.y + self.padding.top;
        content.text = str.clone();
        self.actual_dimensions.height += 20;
        self.content = Some(content);
    }


    fn calculate_position(&mut self, parent: &mut LayoutBox, element_size: usize) {
        self.dimensions.height += self.padding.top + self.padding.bottom;
        if self.box_type == BoxType::Block {
            self.dimensions.y = parent.v_elements + parent.actual_dimensions.y;
            self.dimensions.x = parent.actual_dimensions.x;
            parent.v_elements += self.dimensions.height + self.margin.top + self.margin.bottom;
        } else if self.box_type == BoxType::Inline {
            self.dimensions.y = parent.actual_dimensions.y;
            self.dimensions.x = parent.h_elements + parent.actual_dimensions.x;
            parent.h_elements += self.dimensions.width + self.margin.left + self.margin.right;
        }
    }

    fn calculate_actual_dimensions(&mut self, parent: &mut LayoutBox) {
        self.actual_dimensions.x = self.dimensions.x + self.margin.left + parent.padding.left;
        self.actual_dimensions.y = self.dimensions.y + self.margin.top + parent.padding.top;
        self.actual_dimensions.width = self.dimensions.width - self.margin.left - self.margin.right
            - parent.padding.right * 2;

        self.actual_dimensions.height = self.dimensions.height;
    }


    fn expand_parent_elements_rec(box_: &mut LayoutBox) {
        for child in box_.children.iter_mut() {
            box_.actual_dimensions.height += child.actual_dimensions.height;
            LayoutBox::expand_parent_elements_rec(child);
        }
    }

    fn expand_parent_elements(boxes: &mut Vec<LayoutBox>) {
        for box_ in boxes.iter_mut() {
            if box_.box_type == BoxType::Block {
                // LayoutBox::expand_parent_elements_rec(box_);
            }
        }
    }
}


#[test]
fn test_build_layout_tree() {
    let html1 = r#"
<html>

<head>
    <link rel="stylesheet" type="text/css" href="style.css"></link>
    <title>Example</title>
</head>

<body>
<div id="blue">f</div>
<div class="orange"></div>
<div class="black"></div>
<div class="green"></div>

</body>

</html>
    "#;
    let html2 = r#"
<html>

<head>
    <link rel="stylesheet" type="text/css" href="style.css"></link>
    <title>Example</title>
</head>

<body>
<div id="blue"></div>
<div class="orange"></div>
<div class="black"></div>
<div class="green"></div>

</body>

</html>
    "#;
    let mut parser = HtmlParser::new(html1);
    let nodes = parser.parse_nodes();
    let mut body = nodes[0].children[1].clone();

    let css = r#"
    .orange {
    background-color: #ff6600;
    padding: 20px;
    margin: 50px;
}

#blue{
    background-color: #0a1e77;
    padding: 20px;
    margin: 10px;
}

.black {
    background-color: #000000;
    padding: 20px;
    margin: 30px;
}

.green {
    background-color: #2ebe1a;
    padding: 20px;
    margin: 10px;
}

    "#;
    let mut parser = CssParser::new(&css);
    let stylesheet = parser.parse_stylesheet();
    body.add_styles(&stylesheet);
    let boxes = layout::LayoutBox::build_layout_tree(&body);
    let boxes = crate::render::layout_box_tree_to_vector(boxes);

    let body = &boxes[0];
    let blue = &boxes[1];
    let orange1 = &boxes[3];


    let mut parser = HtmlParser::new(html2);
    let nodes = parser.parse_nodes();
    let mut body = nodes[0].children[1].clone();
    body.add_styles(&stylesheet);
    let boxes = layout::LayoutBox::build_layout_tree(&body);
    let boxes = crate::render::layout_box_tree_to_vector(boxes);
    let orange2 = &boxes[2];

    assert_eq!(orange1, orange2);


    // assert_eq!(boxes.len(), 6);
    // assert_eq!(blue.name, "div");
    // assert_eq!(blue.actual_dimensions.x, 10);
    // assert_eq!(blue.actual_dimensions.y, 10);
    // assert_eq!(blue.actual_dimensions.width, 50);
    // assert_eq!(blue.actual_dimensions.height, 50);
    // // assert_eq!(blue_txt.name, "text");
    // // assert_eq!(blue_txt.actual_dimensions.x, 0);
    // // assert_eq!(blue_txt.actual_dimensions.y, 0);
    // // assert_eq!(blue_txt.actual_dimensions.width, 10);
    // // assert_eq!(blue_txt.actual_dimensions.height, 10);
    // assert_eq!(orange1.name, "div");
    // assert_eq!(orange1.actual_dimensions.x, 50);
    // assert_eq!(orange1.actual_dimensions.y, 100);
    // assert_eq!(orange1.actual_dimensions.width, 50);
    // assert_eq!(orange1.actual_dimensions.height, 50);
    // assert_eq!(orange1.margin.top, 50);
    // assert_eq!(orange1.margin.bottom, 50);
    // assert_eq!(orange1.margin.left, 50);
    // assert_eq!(orange1.margin.right, 50);
    // assert_eq!(orange1.padding.top, 20);
    // assert_eq!(orange1.padding.bottom, 20);
    // assert_eq!(orange1.padding.left, 20);
    // assert_eq!(orange1.padding.right, 20);
    // assert_eq!(orange1.content.width, 10);
}

#[test]
fn test_calculate_position() {
    let html = r#"<html>

<head>
    <link rel="stylesheet" type="text/css" href="style.css"></link>
    <title>Example</title>
</head>

<body>
<div id="blue">ddgsd</div>
<div class="orange"></div>
<div class="green">dfssdf</div>

</body>

</html>"#;

    let css = r#"body {
    color: #772233;
    margin: 10px;
}

.orange {
    background-color: #b46c2b;
    padding: 10px;
    width: 20px;
    margin: 1px;
}

#blue{
    background-color: #093183;
    padding: 20px;
    margin: 10px;
}

.black {
    background-color: #000000;
    padding: 20px;
    margin: 30px;
}

.green {
    background-color: #2ebe1a;
    padding: 20px;
    margin: 10px;
}

.txt {
    background-color: #ffffff;
    font-size: 20px;
    font-family: Arial, Helvetica, sans-serif;
}"#;
    let mut parser = HtmlParser::new(html);
    let nodes = parser.parse_nodes();
    let mut body = nodes[0].children[1].clone();
    let mut parser = CssParser::new(&css);
    let stylesheet = parser.parse_stylesheet();
    body.add_styles(&stylesheet);
    let boxes = LayoutBox::build_layout_tree(&body);
    let boxes = crate::render::layout_box_tree_to_vector(boxes);
    let blue = &boxes[1];
    let orange = &boxes[3];
    let green = &boxes[4];

    assert_eq!(blue.dimensions.width, crate::render::WIDTH as i16);
    assert_eq!(orange.dimensions.width, crate::render::WIDTH as i16);
    assert_eq!(green.dimensions.width, crate::render::WIDTH as i16);
}


