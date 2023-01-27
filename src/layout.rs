use std::fmt;
use crate::css::{Length, PropertyName, PropertyValue};
use crate::{css, dom};
use crate::dom::NodeType;

#[derive(Clone)]
pub struct LayoutBox {
    pub x: i32,
    pub y: i32,
    pub content: Content,
    pub color: Color,
    pub background_color: Color,
    pub name: String,
    pub margin: Indentations,
    pub padding: Indentations,
    pub border: Border,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox>,
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
            y: 0,
            x: 0,
            color: Color::default(),
            margin: Indentations::default(),
            background_color: Color::new(255, 255, 255, 255),
            name: String::from("default"),
            border: Border::default(),
            box_type: BoxType::Block,
            padding: Indentations::default(),
            children: vec![],
        }
    }
}

pub fn build_layout_tree(node: &dom::Node) -> Vec<LayoutBox> {
    let mut root = LayoutBox::default();
    root.box_type = BoxType::Block;
    root.color = Color::default();
    root.margin = Indentations::default();
    root.name = if let NodeType::Element(element_data) = &node.node_type {
        element_data.tag_name.clone()
    } else {
        String::from("root")
    };
    root.padding = Indentations::default();
    root.children = build_layout_tree_helper(&node.children, &mut root, 0);
    vec![root]
}

fn build_box(element: &dom::Node, parent: &LayoutBox) -> LayoutBox {
    let mut box_ = LayoutBox::default();
    for style in &element.styles {
        match style.name {
            PropertyName::Color => {
                if let PropertyValue::Color(color) = &style.value {
                    let color = color.get_rgb();
                    box_.color = Color::new(color.0, color.1, color.1, 255);
                }
            }
            PropertyName::BackgroundColor => {
                if let PropertyValue::Color(color) = &style.value {
                    let color = color.get_rgb();
                    box_.background_color = Color::new(color.0, color.1, color.1, 255);
                }
            },
            PropertyName::Margin => {
                if let PropertyValue::Length(Length::Px(px)) = &style.value {
                    box_.margin = Indentations {
                        top: *px,
                        right: *px,
                        bottom: *px,
                        left: *px,
                    };
                }
            },
            PropertyName::Padding => {
                if let PropertyValue::Length(Length::Px(px)) = &style.value {
                    box_.padding = Indentations {
                        top: *px,
                        right: *px,
                        bottom: *px,
                        left: *px,
                    };
                }
            },
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
            },
            PropertyName::Display => {
                if let PropertyValue::Display(display) = &style.value {
                    box_.box_type = match display {
                        css::DisplayType::Block => BoxType::Block,
                        css::DisplayType::Inline => BoxType::Inline,
                        _ => BoxType::Block,
                    };
                }
            },
            _ => {}
        }
    }

    box_
}

pub fn build_layout_tree_helper(nodes: &Vec<dom::Node>, parent: &LayoutBox, count: i16) -> Vec<LayoutBox> {
    let mut boxes = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        match &node.node_type {
            NodeType::Element(element) => {
                let mut box_ = LayoutBox::default();
                box_.box_type = BoxType::Block;
                box_.name = element.tag_name.clone();
                box_.children = build_layout_tree_helper(&node.children, &mut box_, count + 1);
                boxes.push(box_);
            }
            NodeType::Text(text) => {
                let mut box_ = LayoutBox::default();
                box_.box_type = BoxType::Inline;
                // box_.content.width = 800;
                // box_.content.height = 600;
                box_.color = Color::default();
                box_.margin = Indentations::default();
                box_.padding = Indentations::default();
                box_.children = vec![];
                boxes.push(box_);
            }
            _ => {}
        }
    }
    boxes
}