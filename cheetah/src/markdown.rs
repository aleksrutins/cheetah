use pulldown_cmark::{Options, Parser, html};

pub fn transform(input: String) -> String {
    let options = Options::all();
    let parser = Parser::new_ext(input.as_str(), options);
    let mut html_out = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_out, parser);
    html_out
}
