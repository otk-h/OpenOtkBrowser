use open_otk_browser::resolve_render;

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

    resolve_render(html_input, css_input);
}
