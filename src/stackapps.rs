use crate::html::strip_tags;
use anyhow::Result;
use chrono::prelude::*;
use serde::Deserialize;
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

    pub fn sanitized_title(&self) -> String {
        strip_tags(&self.title)
    }

    pub fn sanitized_excerpt(&self) -> String {
        strip_tags(&self.excerpt)
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
