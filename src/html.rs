use crate::dom::{Node, NodeType, ElementData};

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.next_char();
        self.pos += 1;
        c
    }

    fn consume_while(&mut self, predicate: fn(char) -> bool) -> String {
        let mut result = String::new();
        while let Some(c) = self.next_char() {
            if !predicate(c) {
                break;
            }
            result.push(c);
            self.pos += 1;
        }
        result
    }

    fn eof(&self) -> bool {
        self.next_char().is_none()
    }

    fn parse_element(&mut self) -> Node {
        self.consume(); // <
        let tag_name = self.consume_while(|c| c.is_alphanumeric()); // tag name
        self.consume(); // >
        
        let nodes = self.parse_nodes();
     
        if self.starts_with("</") {
            self.consume(); // <
            self.consume(); // /
            let closing_tag_name = self.consume_while(|c| c.is_alphanumeric()); // tag name
            assert_eq!(tag_name, closing_tag_name);
            self.consume(); // >
        }
     
        Node { children:nodes, node_type: NodeType::Element(ElementData{ tag_name, attributes: vec![] }) }
     
     }

     fn parse_text(&mut self) -> Node {
        Node { children: vec![], node_type: NodeType::Text(self.consume_while(|c| c != '<'))}
    }
    
    fn parse_node(&mut self) -> Node {
        if self.starts_with("<") {
            return self.parse_element();
        }
        self.parse_text()
    }
    
    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
    
        while !self.eof() && !self.starts_with("</") {
            nodes.push(self.parse_node());
        }
    
        nodes
    }
    
}

pub fn parse_html(html: &str) -> Node {
    let mut parser = Parser::new(html);
    parser.parse_node()
}
