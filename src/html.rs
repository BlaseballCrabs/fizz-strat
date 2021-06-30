use ego_tree::iter::Edge;
use scraper::{node::Text, Html, Node};
use selectors::attr::CaseSensitivity::CaseSensitive;
use std::fmt::Write;

pub fn strip_tags(input: &str) -> String {
    let mut buf = String::new();

    let html = Html::parse_fragment(input);
    let root = html.tree.root();

    for edge in root.traverse() {
        match edge {
            Edge::Open(node) => match node.value() {
                Node::Text(Text { text }) => write!(buf, "{}", text).unwrap(),
                Node::Element(elem) if elem.has_class("highlight", CaseSensitive) => {
                    buf += "**";
                }
                _ => {}
            },
            Edge::Close(node) => match node.value() {
                Node::Element(elem) if elem.has_class("highlight", CaseSensitive) => {
                    buf += "**";
                }
                _ => {}
            },
        }
    }

    buf
}
