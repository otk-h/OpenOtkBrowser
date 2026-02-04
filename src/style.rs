use crate::dom;
use crate::css;
use std::collections::HashMap;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a dom::Node,
    pub specified_values: HashMap<String, css::Value>,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(PartialEq)]
pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn value(&self, name: &str) -> Option<css::Value> {
        self.specified_values.get(name).cloned()
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &css::Value) -> css::Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(css::Value::Keyword(s)) => match &*s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }
}

pub fn build_style_tree<'a>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            dom::NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            dom::NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| build_style_tree(child, stylesheet)).collect(),
    }
}

fn specified_values(elem: &dom::ElementData, stylesheet: &css::Stylesheet) -> HashMap<String, css::Value> {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|a: &(css::Specificity, _), b: &(css::Specificity, _)| a.0.cmp(&b.0));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

fn matching_rules<'a>(elem: &dom::ElementData, stylesheet: &'a css::Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

fn match_rule<'a>(elem: &dom::ElementData, rule: &'a css::Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter().find(|selector| matches(elem, selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matches(elem: &dom::ElementData, selector: &css::Selector) -> bool {
    match selector {
        css::Selector::Simple(s) => matches_simple_selector(elem, s)
    }
}

fn matches_simple_selector(elem: &dom::ElementData, selector: &css::SimpleSelector) -> bool {
    if selector.tag.iter().any(|name| elem.tag != *name) {
        return false;
    }
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }
    if selector.class.iter().any(|class| !elem.classes().contains(class.as_str())) {
        return false;
    }
    return true;
}
