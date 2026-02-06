use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag: String,
    pub attributes: HashMap<String, String>,
}

impl Node {
    pub fn text(data: String) -> Node {
        Node {
            node_type: NodeType::Text(data),
            children: Vec::new(),
        }
    }

    pub fn comment(data: String) -> Node {
        Node {
            node_type: NodeType::Comment(data),
            children: Vec::new(),
        }
    }

    pub fn element(tag: String, attrs: HashMap<String, String>, children: Vec<Node>) -> Node {
        Node {
            node_type: NodeType::Element(ElementData {
                tag: tag,
                attributes: attrs,
            }),
            children: children,
        }
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}
