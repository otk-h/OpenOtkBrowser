pub mod lexical;
pub mod stylesheet;
pub mod parser;
pub mod style;

use super::html::dom::Node;
use stylesheet::Stylesheet;

pub fn parse_css(css_input: String) -> stylesheet::Stylesheet {
    parser::CssParser::parse_stylesheet(css_input)
}

pub fn build_styled_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> style::StyledNode<'a> {
    style::StyledNode::build_styled_tree(root, stylesheet)
}
