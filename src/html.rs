pub mod dom;
pub mod parser;

pub fn parse_html(html_input: String) -> dom::Node {
    parser::build_dom_tree(html_input)
}
