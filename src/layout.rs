use crate::dom::{Node, NodeType};
#[derive(Debug, PartialEq)]
pub struct Layout<'a> {
    pub node: &'a Node,
    pub children: Vec<Layout<'a>>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl<'a> Layout<'a> {
    pub fn new(node: &'a Node) -> Layout<'a> {
        Layout { node, children: vec![], x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

pub fn build_layout(node: &Node, x: f32, y: f32) -> Layout {
    let mut layout = Layout::new(node);
    layout.x = x;
    layout.y = y;
    layout.width = 0.0;
    layout.height = 10.0;
    
    match &node.node_type {
        NodeType::Text(_) => {},
        NodeType::Element(_) => {
              let mut child_y = y;
              let mut children = vec![];

              for child in node.children.iter(){
                let child_layout = build_layout(child, x, child_y);
                child_y += child_layout.height;
                children.push(child_layout);
              }
              layout.children = children;
        }
    }
    layout
}

pub fn print_layout(layout: &Layout, indent: usize) {
    let indent_str = "  ".repeat(indent);

    match layout.node.node_type {
        NodeType::Element(ref elem) => {
            println!(
                "{}<{}> at ({}, {}) size: {}x{}",
                indent_str,
                elem.tag_name,
                layout.x,
                layout.y,
                layout.width,
                layout.height
            );
        }
        NodeType::Text(ref text) => {
            println!(
                "{}\"{}\" at ({}, {}) size: {}x{}",
                indent_str,
                text,
                layout.x,
                layout.y,
                layout.width,
                layout.height
            );
        }
    }

    for child in &layout.children {
        print_layout(child, indent + 1);
    }
}