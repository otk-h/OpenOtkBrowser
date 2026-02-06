use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;

mod dom;
mod html;
mod css;
mod style;
mod layout;
mod render;

fn main() {
    let html_input = r#"
        <div class="a">
            <div class="b">
                <div class="c">
                    <div class="d">
                        <div class="e">
                            <div class="f">
                                <div class="g">
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    "#.to_string();

    let css_input = r#"
        * { display: block; padding: 12px; }
        .a { background: #ff0000; }
        .b { background: #ffa500; }
        .c { background: #ffff00; }
        .d { background: #008000; }
        .e { background: #0000ff; }
        .f { background: #4b0082; }
        .g { background: #800080; }
    "#.to_string();

    let node_root = html::parse(html_input.clone());
    let stylesheet = css::parse(css_input.clone());

    // init window system
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("OpenOtkBrowser")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap()
    );
    let context = Context::new(window.clone()).unwrap();
    let mut surface = Surface::new(&context, window.clone()).unwrap();

    let dom_ref = &node_root;
    let style_ref = &stylesheet;

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);
        
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            }
            Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                if new_size.width > 0 && new_size.height > 0 {
                    window.request_redraw();
                }
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let size = window.inner_size();
                if size.width == 0 || size.height == 0 { return; }

                let width = NonZeroU32::new(size.width).unwrap();
                let height = NonZeroU32::new(size.height).unwrap();

                let style_root = style::build_style_tree(dom_ref, style_ref);

                let mut viewport = layout::Dimensions::default();
                viewport.content.width = size.width as f32;
                viewport.content.height = size.height as f32;

                let layout_root = layout::build_layout_tree(&style_root, viewport);

                let pixels = render::render_to_buffer(&layout_root, size.width, size.height);

                surface.resize(width, height).unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.copy_from_slice(&pixels);
                buffer.present().unwrap();
            }
            Event::AboutToWait => {
                window.request_redraw(); 
            }
            _ => {}
        }
    }).unwrap();

}
