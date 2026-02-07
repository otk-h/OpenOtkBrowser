use std::collections::HashMap;

use super::super::html::dom::{Node, NodeType, ElementData};
use super::stylesheet::{Value, Stylesheet, Selector, SimpleSelector};

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub styles: HashMap<String, Value>,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Inline,
    Block,
    None,
}

// pub fn build_styled_tree<'a>(node: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
//     let styles = get_styles(node, stylesheet);
//     let children = node.children.iter()
//         .map(|child| { build_styled_tree(child, stylesheet)})
//         .collect();
//     StyledNode {
//         node,
//         styles,
//         children,
//     }
// }

impl<'a> StyledNode<'a> {
    pub fn build_styled_tree(node: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
        StyledNode {
            node,
            styles: match &node.node_type {
                NodeType::Element(elem) => Self::specified_values(elem, stylesheet),
                NodeType::Text(_) => HashMap::new(),
                NodeType::Comment(_) => HashMap::new(),
            },
            children: node.children.iter()
                .map(|child| StyledNode::build_styled_tree(child, stylesheet))
                .collect(),
        }
    }

    fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> HashMap<String, Value> {
        let mut values = HashMap::new();
        let mut rules = stylesheet.rules.iter()
            .filter_map(|rule| {
                rule.selectors.iter()
                    .find(|selector| matches(elem, selector))
                    .map(|selector| (selector.specificity(), rule))
            })
            .collect::<Vec<_>>();

        rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
        for (_, rule) in rules {
            for declaration in &rule.declarations {
                values.insert(declaration.name.clone(), declaration.value.clone());
            }
        }
        values
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }

    pub fn value(&self, name: &str) -> Option<Value> {
        self.styles.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

//     pub fn get_number(&self, name: &str) -> Option<&f32> {
//         self.get_value(name).and_then(|v| v.as_number())
//     }

//     pub fn get_px(&self, name: &str) -> Option<&f32> {
//         self.get_value(name).and_then(|v| v.to_px())
//     }

//     pub fn get_color(&self, name: &str) -> Option<&Color> {
//         self.get_value(name).and_then(|v| v.as_color())
//     }

// }

// fn get_styles(node: &Node, stylesheet: &Stylesheet) -> HashMap<String, Value> {
//     let mut styles = HashMap::new();
    
//     let mut rules = Vec::new();

//     for rule in &stylesheet.rules {
//         for selector in &rule.selectors {
//             if selector_matches(selector, node) {
//                 rules.push((selector.specificity(), rule));
//                 break;
//             }
//         }
//     }

//     rules.sort_by(|a, b| a.0.cmp(&b.0));
    
//     for (_, rule) in rules {
//         for declaration in &rule.declarations {
//             styles.insert(declaration.name.clone(), declaration.value.clone());
//         }
//     }

//     return styles;
}

// fn selector_matches(selector: &Selector, node: &Node) -> bool {
//     let NodeType::Element(ref elem) = node.node_type else {
//         return false;
//     };

//     let id = elem.attributes.get("id").map(|c| c.to_string());
//     let classes: Vec<String> = elem.attributes.get("class")
//         .map(|s| s.split_whitespace().map(|c| c.to_string()).collect())
//         .unwrap_or_default();

//     return selector.matches(&elem.tag_name, id, &classes);
// }

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(simple) => {
            matches_simple(elem, simple)
        },
        Selector::Descendant(_, _) => {
            false
        },
        Selector::Child(_, _) => {
            false
        },
        Selector::AdjacentSibling(_, _) => {
            false
        },
        Selector::GeneralSibling(_, _) => {
            false
        },
    }
}

fn matches_simple(elem: &ElementData, simple: &SimpleSelector) -> bool {
    let elem_classes: Vec<&str> = elem.attributes.get("class")
                                    .map(|s| s.split_whitespace().collect())
                                    .unwrap_or_default();

    if simple.tag.iter().any(|name| name != &elem.tag)
        || simple.id.iter().any(|id| Some(id) != elem.attributes.get("id"))
        || simple.class.iter().any(|class| !elem_classes.contains(&class.as_str()))
    {
        return false;
    }
    
    return true
}