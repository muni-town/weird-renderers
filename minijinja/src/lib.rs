use common::*;
use minijinja::*;

render_function!(render);

fn render_markdown(m: String) -> String {
    markdown::to_html_with_options(&m, &markdown::Options::gfm())
        .expect("Couldn't render markdown.")
}

fn render(profile_data: ProfileData, theme: &mut [u8]) -> String {
    let mut env = Environment::new();
    env.add_filter("markdown", render_markdown);
    let template = core::str::from_utf8(theme).unwrap();
    env.add_template("index", template).unwrap();
    let tpl = env.get_template("index").unwrap();
    tpl.render(profile_data).unwrap()
}
