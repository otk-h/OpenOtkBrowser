pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod render;

pub fn resolve_render(html_input: String, css_input: String) {
    let node_root = html::parse(html_input);
    let stylesheet = css::parse(css_input);
    let style_root = style::build_style_tree(&node_root, &stylesheet);

    let mut viewport = layout::Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    let layout_root = layout::build_layout_tree(&style_root, viewport);

    let canvas = render::paint(&layout_root, viewport.content);
    canvas.save_png("output.png").expect("Failed to save PNG");
}
