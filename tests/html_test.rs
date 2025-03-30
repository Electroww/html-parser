#[cfg(test)]
mod tests {
    use browser::html::*;
    use browser::dom::*;
    
    #[test] 
    fn test_parse_html_simple_node() {
        let result = parse_html("<div></div>");
        println!("{:?}\n", result);
        assert_eq!(result, Node { children: vec![], node_type: NodeType::Element(ElementData{ tag_name: "div".to_string(), attributes: vec![] }) });
    }

    #[test] 
    fn test_parse_html_simple_node_with_text() {
        let result = parse_html("<div>Hello</div>");
        println!("{:?}\n", result);
        assert_eq!(result, Node { children: vec![Node { children: vec![], node_type: NodeType::Text("Hello".to_string()) }], node_type: NodeType::Element(ElementData{ tag_name: "div".to_string(), attributes: vec![] }) });
    }

    #[test] 
    fn test_parse_html_simple_node_with_child() {
        let result = parse_html("<div><h1></h1></div>");
        println!("{:?}\n", result);
        assert_eq!(result, Node { children: vec![Node { children: vec![], node_type: NodeType::Element(ElementData{ tag_name: "h1".to_string(), attributes: vec![] }) }], node_type: NodeType::Element(ElementData{ tag_name: "div".to_string(), attributes: vec![] }) });
    }

    #[test] 
    fn test_parse_html_simple_node_with_2child() {
        let result = parse_html("<div><h1></h1><h2></h2></div>");
        println!("{:?}\n", result);
        assert_eq!(result.children.len(), 2);
    }

}
