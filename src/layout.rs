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
            background_color: Color::new(0, 0, 0, 255),
            name: String::from("default"),
            box_type: BoxType::Block,
            padding: Indentations::default(),
            children: vec![],
            v_elements: 0,
        }
    }
}

impl LayoutBox {
    pub fn build_layout_tree(node: &dom::Node) -> Vec<LayoutBox> {
        let mut body = LayoutBox::default();
        body.dimensions.width = crate::render::WIDTH as i16;
        body.box_type = BoxType::Block;
        body.name = if let NodeType::Element(element_data) = &node.node_type {
            element_data.tag_name.clone()
        } else {
            panic!("Root node must be an element");
        };
        body.children = LayoutBox::build_layout_tree_helper(&node.children, &mut body, 0);
        vec![body]
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
                    parent.set_content(&text);
                }
                _ => {}
            }
        }
        boxes
    }

    fn build_box(element: &dom::Node, parent: &mut LayoutBox, element_data: &ElementData, element_number: usize) -> LayoutBox {
        let mut box_ = LayoutBox::default();
        for (name, value) in &element.styles {
            match &name {
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
                            box_.dimensions.width = parent.dimensions.width * (*percent as i16 / 100);
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
                s => { println!("kurwa{:?}", s) }
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
        self.actual_dimensions.width += 20;
        self.content = Some(content);
    }


    fn calculate_position(&mut self, parent: &mut LayoutBox, element_size: usize) {
        self.dimensions.height = self.padding.top + self.padding.bottom;
        // if self.content.is_some() {
        //     parent.dimensions.height += 10;
        // }

        self.dimensions.y = parent.v_elements + parent.dimensions.y;
        self.dimensions.x = parent.dimensions.x;

        if element_size != 0 {
            if let Some(before) = parent.children.get(element_size - 1) {
                self.actual_dimensions.y = before.margin.bottom + self.dimensions.y + self.margin.top;
            }
        }
        // 20 is fixed text size
        parent.v_elements += self.dimensions.height + self.margin.top + self.margin.bottom + 20;
    }
    fn calculate_actual_dimensions(&mut self, parent: &mut LayoutBox) {
        self.actual_dimensions.x = self.dimensions.x + self.margin.left + parent.padding.left;
        self.actual_dimensions.y = self.dimensions.y + self.margin.top + parent.padding.top;
        self.actual_dimensions.width = self.dimensions.width - self.margin.left - self.margin.right;
        self.actual_dimensions.height = self.dimensions.height;
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
    let boxes = crate::render::layout_box_tree_to_vector(boxes[0].clone());

    let body = &boxes[0];
    let blue = &boxes[1];
    let orange1 = &boxes[3];


    let mut parser = HtmlParser::new(html2);
    let nodes = parser.parse_nodes();
    let mut body = nodes[0].children[1].clone();
    body.add_styles(&stylesheet);
    let boxes = layout::LayoutBox::build_layout_tree(&body);
    let boxes = crate::render::layout_box_tree_to_vector(boxes[0].clone());
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
    let boxes = crate::render::layout_box_tree_to_vector(boxes[0].clone());
    let blue = &boxes[1];
    let orange = &boxes[3];
    let green = &boxes[4];

    assert_eq!(blue.dimensions.width, crate::render::WIDTH as i16);
    assert_eq!(orange.dimensions.width, crate::render::WIDTH as i16);
    assert_eq!(green.dimensions.width, crate::render::WIDTH as i16);
}


