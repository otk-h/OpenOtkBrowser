pub mod dom;
pub mod html;
pub mod css;
pub mod style;

pub fn resolve_style(html_input: String, css_input: String) {
    let root_node = html::parse(html_input);
    let stylesheet = css::parse(css_input);
    let styled_root = style::style_tree(&root_node, &stylesheet);
    println!("{:#?}", styled_root);
}
