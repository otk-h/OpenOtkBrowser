use open_otk_browser::*;

#[test]
fn test_html_parsing() {
    let html_input = r#"
        <div id="main" class="container">
            <h1>Hello Rust Browser</h1>
            <p>test_string</p>
        </div>
    "#.to_string();

    let root_node = crate::html::parse(html_input);
    println!("{:#?}", root_node);
}