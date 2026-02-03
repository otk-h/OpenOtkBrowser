use crate::dom::Node;
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

pub fn parse(input: String) -> Node {
    Parser::new(input).parse()
}

impl Parser {
    fn new(input: String) -> Self {
        Parser { pos: 0, input }
    }

    fn parse(&mut self) -> Node {
        let mut children = self.parse_nodes();

        if children.len() == 1 {
            return children.remove(0)
        } else {
            return Node::element("html".to_string(), HashMap::new(), children)
        }
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;   
            }
            nodes.push(self.parse_node());
        }

        return nodes;
    }

    fn parse_node(&mut self) -> Node {
        if self.starts_with("<") {
            return self.parse_element()
        } else {
            return self.parse_text()
        }
    }

    fn parse_element(&mut self) -> Node {
        assert!(self.consume_char() == '<');
        let tag = self.parse_tag();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        let children = self.parse_nodes();
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag() == tag);
        assert!(self.consume_char() == '>');

        return Node::element(tag, attrs, children);
    }

    fn parse_tag(&mut self) -> String {
        self.consume_while(|c| c.is_ascii_alphanumeric())
    }

    fn parse_text(&mut self) -> Node {
        Node::text(self.consume_while(|c| c != '<'))
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_tag();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    // assistant functions

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn consume_while<F>(&mut self, test: F) -> String 
    where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn eof(&mut self) -> bool {
        return self.pos >= self.input.len();
    }

    fn consume_char(&mut self) -> char {
        let ch = self.input[self.pos..].chars().next().unwrap();
        self.pos += ch.len_utf8();
        return ch;
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }
}
