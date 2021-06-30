use anyhow::Result;
use chrono::prelude::*;
use ego_tree::iter::Edge;
use scraper::{node::Text, Html, Node};
use selectors::attr::CaseSensitivity::CaseSensitive;
use serde::Deserialize;
use std::fmt::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct SearchExcerpt {
    pub title: String,
    pub excerpt: String,
    pub question_id: u64,

    #[serde(with = "serde_with::chrono::datetime_utc_ts_seconds_from_any")]
    pub creation_date: DateTime<Utc>,
}

impl SearchExcerpt {
    pub fn question_url(&self, site: &str) -> String {
        format!("https://{}/questions/{}", site, self.question_id)
    }

    pub fn sanitized_excerpt(&self) -> String {
        let mut buf = String::new();

        let html = Html::parse_fragment(&self.excerpt);
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
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub items: Vec<T>,

    #[serde(default)]
    pub backoff: u64,
}

impl<T> Response<T> {
    pub fn delayed(self) -> Vec<T> {
        if self.backoff > 0 {
            thread::sleep(Duration::from_secs(self.backoff));
        }

        self.items
    }
}

pub fn search(site: &str, query: &str, key: &str) -> Result<Vec<SearchExcerpt>> {
    let excerpts = attohttpc::get("https://api.stackexchange.com/2.3/search/excerpts")
        .param("order", "desc")
        .param("sort", "relevance")
        .param("pagesize", 100)
        .param("site", site)
        .param("q", query)
        .param("key", key)
        .send()?
        .error_for_status()?
        .json::<Response<_>>()?
        .delayed();

    Ok(excerpts)
}
