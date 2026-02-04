use crate::layout;
use crate::css;
use tiny_skia;

pub enum DisplayCommand {
    SolidColor(css::Color, layout::Rect),
    // todo
    Image,
    Text,
    Border,
}

pub fn paint(layout_root: &layout::LayoutBox, bounds: layout::Rect) -> tiny_skia::Pixmap {
    let mut pixmap = tiny_skia::Pixmap::new(bounds.width as u32, bounds.height as u32).unwrap();
    let display_list = build_display_list(layout_root);

    for cmd in display_list {
        match cmd {
            DisplayCommand::SolidColor(color, rect) => {
                let skia_color = tiny_skia::Color::from_rgba8(color.r, color.g, color.b, color.a);
                
                let mut paint = tiny_skia::Paint::default();
                paint.set_color(skia_color);
                paint.anti_alias = true;

                if let Some(skia_rect) = tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width, rect.height) {
                    let mut pb = tiny_skia::PathBuilder::new();
                    pb.push_rect(skia_rect);
                    let path = pb.finish().unwrap();

                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None);
                }
            }
            // todo
            _ => {}
        }
    }
    return pixmap;
}

fn build_display_list(layout_root: &layout::LayoutBox) -> Vec<DisplayCommand> {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut Vec<DisplayCommand>, lbox: &layout::LayoutBox) {
    if let layout::BoxType::BlockNode(style_node) = &lbox.box_type {
        if let Some(css::Value::ColorValue(color)) = style_node.value("background-color") {
            list.push(DisplayCommand::SolidColor(color.clone(), lbox.dimensions.content.clone()));
        }
    }
    for child in &lbox.children {
        render_layout_box(list, child);
    }
}
