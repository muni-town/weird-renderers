use common::*;
use minijinja::*;

render_function!(render);

fn render_markdown(m: String) -> String {
    markdown::to_html_with_options(&m, &markdown::Options::gfm())
        .expect("Couldn't render markdown.")
}

fn render_markdown_text(m: String) -> String {
    let ast = markdown::to_mdast(&m, &markdown::ParseOptions::gfm())
        .expect("Couldn't render markdown text.");
    let mut out = String::with_capacity(m.capacity());
    accumulate_plaintext(&mut out, ast);
    out
}

fn accumulate_plaintext(out: &mut String, node: markdown::mdast::Node) {
    match node {
        markdown::mdast::Node::Root(x) => x
            .children
            .into_iter()
            .for_each(|node| accumulate_plaintext(out, node)),
        markdown::mdast::Node::Blockquote(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
        }),
        markdown::mdast::Node::FootnoteDefinition(_) => (),
        markdown::mdast::Node::MdxJsxFlowElement(_) => (),
        markdown::mdast::Node::List(x) => {
            x.children.into_iter().for_each(|node| {
                accumulate_plaintext(out, node);
            });
        }
        markdown::mdast::Node::MdxjsEsm(_) => (),
        markdown::mdast::Node::Toml(_) => (),
        markdown::mdast::Node::Yaml(_) => (),
        markdown::mdast::Node::Break(_) => out.push('\n'),
        markdown::mdast::Node::InlineCode(x) => {
            out.push_str(&x.value);
        }
        markdown::mdast::Node::InlineMath(x) => {
            out.push_str(&x.value);
        }
        markdown::mdast::Node::Delete(_) => (),
        markdown::mdast::Node::Emphasis(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
        }),
        markdown::mdast::Node::MdxTextExpression(_) => (),
        markdown::mdast::Node::FootnoteReference(_) => (),
        markdown::mdast::Node::Html(_) => (),
        markdown::mdast::Node::Image(x) => {
            out.push_str(&x.alt);
        }
        markdown::mdast::Node::ImageReference(x) => {
            out.push_str(&x.alt);
        }
        markdown::mdast::Node::MdxJsxTextElement(_) => (),
        markdown::mdast::Node::Link(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
        }),
        markdown::mdast::Node::LinkReference(_) => (),
        markdown::mdast::Node::Strong(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
        }),
        markdown::mdast::Node::Text(x) => out.push_str(&x.value),
        markdown::mdast::Node::Code(x) => {
            out.push_str(&x.value);
        }
        markdown::mdast::Node::Math(_) => (),
        markdown::mdast::Node::MdxFlowExpression(_) => (),
        markdown::mdast::Node::Heading(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
            out.push_str("\n\n");
        }),
        markdown::mdast::Node::Table(_) => (),
        markdown::mdast::Node::ThematicBreak(_) => (),
        markdown::mdast::Node::TableRow(_) => (),
        markdown::mdast::Node::TableCell(_) => (),
        markdown::mdast::Node::ListItem(x) => x.children.into_iter().for_each(|node| {
            accumulate_plaintext(out, node);
        }),
        markdown::mdast::Node::Definition(_) => (),
        markdown::mdast::Node::Paragraph(x) => {
            x.children.into_iter().for_each(|node| {
                accumulate_plaintext(out, node);
            });
            out.push_str("\n\n");
        }
    }
}

fn render(profile_data: ProfileData, theme: &mut [u8]) -> String {
    let mut env = Environment::new();
    env.add_filter("markdown", render_markdown);
    env.add_filter("markdown_text", render_markdown_text);
    let template = core::str::from_utf8(theme).unwrap();
    env.add_template("index", template).unwrap();
    let tpl = env.get_template("index").unwrap();
    tpl.render(profile_data).unwrap()
}
