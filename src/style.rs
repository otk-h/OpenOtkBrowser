use crate::dom;
use crate::css;
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, css::Value>;

#[derive(Debug)]
pub struct StyledNode<'a> {
    pub node: &'a dom::Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

type MatchedRule<'a> = (css::Specificity, &'a css::Rule);

impl<'a> StyledNode<'a> {

}
