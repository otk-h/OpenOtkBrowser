use crate::style;
use crate::css;

#[derive(Default, Debug, Clone, Copy)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Debug)]
pub enum BoxType<'a> {
    BlockNode(&'a style::StyledNode<'a>),
    InlineNode(&'a style::StyledNode<'a>),
    AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
    pub fn new(node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
        LayoutBox {
            dimensions: Dimensions::default(),
            box_type: match node.display() {
                style::Display::Block => BoxType::BlockNode(node),
                style::Display::Inline => BoxType::InlineNode(node),
                style::Display::None => panic!("Root node has display: none"),
            },
            children: Vec::new(),
        }
    }

    fn get_style_node(&self) -> &'a style::StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block box has no style node")
        }
    }
}

pub fn build_layout_tree<'a>(node: &'a style::StyledNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
    containing_block.content.height = 0.0;
    let mut root_box = layout_tree(node);
    root_box.layout(containing_block);
    return root_box;
}

fn layout_tree<'a>(style_node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(style_node);
    for child in &style_node.children {
        match child.display() {
            style::Display::Block => root.children.push(layout_tree(child)),
            style::Display::Inline => root.get_inline_container().children.push(layout_tree(child)),
            style::Display::None => {} 
        }
    }
    return root;
}

impl LayoutBox<'_> {
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => {}
            BoxType::AnonymousBlock => {}
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        self.calculate_block_width(containing_block);
        self.calculate_block_position(containing_block);
        self.layout_block_children();
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let auto = css::Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        let zero = css::Value::Length(0.0, css::Unit::Px);
        
        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total: f32 = [&margin_left, &margin_right, &border_left, &border_right, &padding_left, &padding_right, &width]
                        .iter().map(|v| v.to_px()).sum();

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = css::Value::Length(0.0, css::Unit::Px);
            }
            if margin_right == auto {
                margin_right = css::Value::Length(0.0, css::Unit::Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = css::Value::Length(margin_right.to_px() + underflow, css::Unit::Px);
            }
            (false, false, true) => {
                margin_right = css::Value::Length(underflow, css::Unit::Px);
            }
            (false, true, false) => {
                margin_left = css::Value::Length(underflow, css::Unit::Px);
            }
            (true, _, _) => {
                if margin_left == auto { margin_left = css::Value::Length(0.0, css::Unit::Px); }
                if margin_right == auto { margin_right = css::Value::Length(0.0, css::Unit::Px); }

                if underflow >= 0.0 {
                    width = css::Value::Length(underflow, css::Unit::Px);
                } else {
                    width = css::Value::Length(0.0, css::Unit::Px);
                    margin_right = css::Value::Length(margin_right.to_px() + underflow, css::Unit::Px);
                }
            }
            (false, true, true) => {
                margin_left = css::Value::Length(underflow / 2.0, css::Unit::Px);
                margin_right = css::Value::Length(underflow / 2.0, css::Unit::Px);
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();
        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();
        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();
        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;
        let zero = css::Value::Length(0.0, css::Unit::Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();
        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();
        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x +
                      d.margin.left + d.border.left + d.padding.left;

        d.content.y = containing_block.content.height + containing_block.content.y +
                      d.margin.top + d.border.top + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        for child in &mut self.children {
            child.layout(self.dimensions);
            self.dimensions.content.height += child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        if let Some(css::Value::Length(h, css::Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }

    fn get_inline_container(&mut self) -> &mut Self {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox { box_type: BoxType::AnonymousBlock,..}) => {}
                    _ => self.children.push(LayoutBox { 
                        dimensions: Dimensions::default(),
                        box_type: BoxType::AnonymousBlock,
                        children: Vec::new(),
                    }),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

impl Rect {
    pub fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

impl Dimensions {
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}
