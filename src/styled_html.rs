use crate::css::Property;
use crate::html::Node;

pub struct StyledNode {
    pub node: Node,
    pub styles: Vec<Property>,
}

pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}
