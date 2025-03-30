#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
}

pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attrs: Vec<(String, String)>, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData{ tag_name: name, attributes: attrs })
    }
}