mod dom;
mod html;

fn main() {
    let html_input = r#"
        <div id="main" class="container">
            <h1>Hello Rust Browser</h1>
            <p>test_string</p>
        </div>
    "#.to_string();

    println!("start parsing...");

    let root_node = html::parse(html_input);
    println!("{:#?}", root_node);
}