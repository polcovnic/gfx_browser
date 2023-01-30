use std::collections::hash_set::Union;
use std::fmt;
use crate::css::{Length, PropertyName, PropertyValue};
use crate::{css, dom};
use crate::dom::{ElementData, NodeType};

#[derive(Clone)]
pub struct LayoutBox {
    pub dimensions: Dimensions,
    pub content: Content,
    pub element_type: ElementType,
    pub color: Color,
    pub background_color: Color,
    pub name: String,
    pub margin: Indentations,
    pub padding: Indentations,
    pub border: Border,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox>,
    v_elements: i16,
}


#[derive(Clone, Default)]
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

#[derive(Clone)]
pub enum ElementType {
    Text,
    Element,
}

#[derive(Clone)]
pub enum BoxType {
    Block,
    Inline,
}

#[derive(Clone, Default)]
pub struct Content {
    pub height: i16,
    pub width: i16,
    pub text: Option<String>,
}

#[derive(Clone, Default)]
pub struct Indentations {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

#[derive(Clone, Default)]
pub struct Border {
    pub width: i16,
    pub color: Color,
    pub style: css::BorderStyle,
}


#[derive(Clone, Default)]
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
            content: Content::default(),
            dimensions: Dimensions::default(),
            color: Color::new(0, 0, 0, 255),
            margin: Indentations::default(),
            element_type: ElementType::Element,
            background_color: Color::new(0, 0, 0, 255),
            name: String::from("default"),
            border: Border::default(),
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
        body.box_type = BoxType::Block;
        body.name = if let NodeType::Element(element_data) = &node.node_type {
            element_data.tag_name.clone()
        } else {
            panic!("Root node must be an element");
        };
        body.children = LayoutBox::build_layout_tree_helper(&node.children, &mut body, 0);
        vec![body]
    }


    fn build_layout_tree_helper(nodes: &Vec<dom::Node>, parent: &mut LayoutBox, count: i16) -> Vec<LayoutBox> {
        let mut boxes = Vec::new();
        for (i, node) in nodes.iter().enumerate() {
            match &node.node_type {
                NodeType::Element(element) => {
                    let mut box_ = LayoutBox::build_box(node, parent, &element);

                    box_.children = LayoutBox::build_layout_tree_helper(&node.children, &mut box_, count + 1);
                    boxes.push(box_);
                }
                NodeType::Text(text) => {
                    let text_box = LayoutBox::build_text_layout_box(node, parent, text.clone());
                    boxes.push(text_box);
                }
                _ => {}
            }
        }
        boxes
    }

    fn build_text_layout_box(node: &dom::Node, parent: &mut LayoutBox, text: String) -> LayoutBox {
        let mut text_box = LayoutBox::default();
        text_box.name = String::from("text");
        text_box.element_type = ElementType::Text;
        text_box.color = Color::new(0, 0, 0, 255);
        text_box.content.text = Some(text);
        text_box.calculate_position(parent);
        text_box
    }

    fn build_box(element: &dom::Node, parent: &mut LayoutBox, element_data: &ElementData) -> LayoutBox {
        let mut box_ = LayoutBox::default();
        for style in &element.styles {
            match &style.name {
                PropertyName::Color => {
                    if let PropertyValue::Color(color) = &style.value {
                        let color = color.get_rgb();
                        box_.color = Color::new(color.0, color.1, color.2, 255);
                    }
                }
                PropertyName::BackgroundColor => {
                    if let PropertyValue::Color(color) = &style.value {
                        let color = color.get_rgb();
                        box_.background_color = Color::new(color.0, color.1, color.2, 255);
                    }
                }
                PropertyName::Margin => {
                    if let PropertyValue::Length(Length::Px(px)) = &style.value {
                        box_.margin = Indentations {
                            top: *px,
                            right: *px,
                            bottom: *px,
                            left: *px,
                        };
                    }
                }
                PropertyName::Padding => {
                    if let PropertyValue::Length(Length::Px(px)) = &style.value {
                        box_.padding = Indentations {
                            top: *px,
                            right: *px,
                            bottom: *px,
                            left: *px,
                        };
                    }
                }
                PropertyName::Border => {
                    if let PropertyValue::Border(border) = &style.value {
                        if let Length::Px(px) = border.width {
                            let color = border.color.get_rgb();
                            let color = Color::new(color.0, color.1, color.1, 255);
                            box_.border = Border {
                                width: px,
                                color,
                                style: border.style.clone(),
                            };
                        }
                    }
                }
                PropertyName::Display => {
                    if let PropertyValue::Display(display) = &style.value {
                        box_.box_type = match display {
                            css::DisplayType::Block => BoxType::Block,
                            css::DisplayType::Inline => BoxType::Inline,
                            _ => BoxType::Block,
                        };
                    }
                }
                s => { println!("{:?}", s) }
            }
        }
        box_.name = element_data.tag_name.clone();
        box_.box_type = BoxType::Block;
        box_.calculate_position(parent);

        box_
    }


    fn calculate_position(&mut self, parent: &mut LayoutBox) {
        self.content.width = 10;
        self.content.height = 10;
        self.dimensions.height = self.padding.top + self.content.height + self.padding.bottom;
        self.dimensions.width = self.padding.left + self.content.width + self.padding.right;

        self.dimensions.y = parent.v_elements + parent.dimensions.y;
        self.dimensions.x = parent.dimensions.x + parent.padding.left;
        parent.v_elements += self.dimensions.height;
    }
}


