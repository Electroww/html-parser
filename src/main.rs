
mod dom;
mod html;
mod layout;

use dom::*;
use html::*;
use layout::*;

fn print_node(node: &Node, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match &node.node_type {
        NodeType::Text(s) => println!("{}Text: {}", indent_str, s),
        NodeType::Element(elem) => {
            println!("{}Element: <{}>", indent_str, elem.tag_name);
            for child in &node.children {
                print_node(child, indent + 1 );
            }
        }
    }
}
fn main() {
    let html = std::fs::read_to_string("src/index.html").expect("Failed to read file");
    let dom = parse_html(&html);
    print_node(&dom, 0);
    let layout = build_layout(&dom, 0.0, 0.0);
    print_layout(&layout, 0);
}
