use std::fmt;
use crate::dom;
use crate::dom::NodeType;

#[derive(Clone)]
pub struct LayoutBox {
    pub content: Content,
    pub color: Color,
    pub name: String,
    pub margin: Margin,
    pub padding: Padding,
    pub box_type: BoxType,
    pub children: Option<Vec<LayoutBox>>,
}
impl fmt::Debug for LayoutBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {{", self.name)?;
        for child in self.children.as_ref().unwrap_or(&vec![]).iter() {
            write!(f, "{:?}", child)?;
        }
        write!(f, "}}")
    }
}

#[derive(Clone)]
pub enum BoxType {
    BlockBox,
    InlineBox,
    BlockInlineBox,
}

#[derive(Clone)]
pub struct Content {
    pub height: i16,
    pub width: i16,
    pub x: i16,
    pub y: i16,
    pub text: Option<String>,
}

#[derive(Clone)]
pub struct Margin {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

#[derive(Clone)]
pub struct Padding {
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
    pub left: i16,
}

#[derive(Clone)]
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
}

impl Default for LayoutBox {
    fn default() -> LayoutBox {
        LayoutBox {
            content: Content::default(),
            color: Color::default(),
            margin: Margin::default(),
            name: String::from("default"),
            box_type: BoxType::BlockBox,
            padding: Padding::default(),
            children: None,
        }
    }
}

impl Default for Content {
    fn default() -> Content {
        Content {
            x: 100,
            y: 200,
            width: 300,
            height: 300,
            text: None,
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 255,
            g: 128,
            b: 64,
            a: 255,
        }
    }
}

impl Default for Margin {
    fn default() -> Margin {
        Margin {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

impl Default for Padding {
    fn default() -> Padding {
        Padding {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
        }
    }
}

pub fn build_layout_tree<'a>(node: &'a dom::Node) -> Vec<LayoutBox> {
    let mut root = LayoutBox::default();
    root.box_type = BoxType::BlockBox;
    root.color = Color::default();
    root.margin = Margin::default();
    root.name = if let NodeType::Element(element_data) = &node.node_type {
        element_data.tag_name.clone()
    } else {
        String::from("root")
    };
    root.padding = Padding::default();
    root.children = build_layout_tree_helper(&node.children, &mut root, 0);
    vec![root]
}

pub fn build_layout_tree_helper(nodes: &Vec<dom::Node>, parent: &mut LayoutBox, count: i16) -> Option<Vec<LayoutBox>> {
    let mut boxes = Vec::new();
    for node in nodes {
        match &node.node_type {
            dom::NodeType::Element(element) => {
                let mut box_ = LayoutBox::default();
                box_.box_type = BoxType::BlockBox;
                box_.content.x = 100 * count;
                box_.content.y = 100 * count;
                box_.name = element.tag_name.clone();
                box_.children = build_layout_tree_helper(&node.children, &mut box_, count + 1);
                boxes.push(box_.clone());
                if let Some(children) = &mut parent.children {
                    children.push(box_);
                }
            }
            dom::NodeType::Text(text) => {
                let mut box_ = LayoutBox::default();
                box_.box_type = BoxType::InlineBox;
                box_.content.x = 0;
                box_.content.y = 0;
                // box_.content.width = 800;
                // box_.content.height = 600;
                box_.color = Color::default();
                box_.margin = Margin::default();
                box_.padding = Padding::default();
                box_.children = None;
                boxes.push(box_);
            }
            _ => {}
        }
    }
    if boxes.is_empty() {
        None
    } else {
        Some(boxes)
    }
}