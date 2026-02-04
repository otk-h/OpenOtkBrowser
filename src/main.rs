use open_otk_browser::resolve_style;

fn main() {
    let html_input = r#"
        <div id="main" class="container">
            <h1 class="title">Hello Rust Browser</h1>
            <p>通过解析 CSS 渲染样式。</p>
        </div>
    "#.to_string();

    let css_input = r#"
        div { display: block; width: 400px; }
        .container { background: #ffffff; }
        h1 { color: #ff0000; font-size: 24px; }
        .title { color: #0000ff; }
    "#.to_string();

    resolve_style(html_input, css_input);
}
