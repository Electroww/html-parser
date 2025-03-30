#[cfg(test)]
mod tests {
    use browser::layout::*;
    use browser::dom::*;

    #[test] 
    fn test_build_layout() {
        let node = Node { 
            children: vec![], 
            node_type: NodeType::Element(ElementData{ 
                tag_name: "div".to_string(), 
                attributes: vec![] 
            }) 
        };
        
        let result = build_layout(&node, 0.0, 0.0);
        println!("{:?}\n", result);
        assert_eq!(result, Layout { node: &node, children: vec![], x: 0.0, y: 0.0, width: 0.0, height: 10.0 });
    }

    #[test]
    fn test_build_layout_with_child() {
        let node = Node { 
            children: vec![Node { 
                children: vec![], 
                node_type: NodeType::Element(ElementData{ 
                    tag_name: "h1".to_string(), 
                    attributes: vec![] 
                }) 
            }], 
            node_type: NodeType::Element(ElementData{ 
                tag_name: "div".to_string(), 
                attributes: vec![] 
            }) 
        };
        
        let result = build_layout(&node, 0.0, 10.0);
        println!("{:?}\n", result);
        assert_eq!(result, Layout { node: &node, children: vec![Layout { node: &node.children[0], children: vec![], x: 0.0, y: 10.0, width: 0.0, height: 10.0 }], x: 0.0, y: 10.0, width: 0.0, height: 10.0 });
    }
}