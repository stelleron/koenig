mod template;

// Convert a Markdown file to HTML
fn convert_md_to_html(file_path : &str) {
    print!("{file_path}");
}

fn main() {
    // Create parser with example Markdown text.
    let markdown_input = "hello word";
    let parser = pulldown_cmark::Parser::new(markdown_input);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    convert_md_to_html(&html_output);
}
