use anyhow::{Context, Result};
use discord::*;
use log::*;
use rand::prelude::*;
use stackapps::*;
use std::thread;
use std::time::Duration;

pub mod discord;
pub mod html;
pub mod stackapps;

pub fn get_excerpts(key: &str) -> Result<impl Iterator<Item = (&'static str, SearchExcerpt)>> {
    fn search_with_site(
        site: &'static str,
        query: &str,
        key: &str,
    ) -> Result<impl Iterator<Item = (&'static str, SearchExcerpt)>> {
        Ok(search(site, query, key)?
            .into_iter()
            .map(move |x| (site, x)))
    }

    let cooking = search_with_site("cooking.stackexchange.com", "carbonation", key)?;
    let chemistry = search_with_site("chemistry.stackexchange.com", "carbonation", key)?;

    Ok(cooking.into_iter().chain(chemistry))
}

pub fn pick_excerpt(key: &str) -> Result<(&'static str, SearchExcerpt)> {
    let excerpts = get_excerpts(key)?;

    excerpts
        .choose(&mut rand::thread_rng())
        .context("No results!")
}

pub fn make_message(site: &str, excerpt: SearchExcerpt) -> Message {
    let title = excerpt.sanitized_title();
    let description = excerpt.sanitized_excerpt();
    let url = excerpt.question_url(site);

    Message {
        embeds: vec![Embed {
            title,
            description,
            url,
            timestamp: excerpt.creation_date,
            author: EmbedAuthor {
                name: site.to_string(),
                url: format!("https://{}", site),
            },
        }],
    }
}

pub fn get_message(key: &str) -> Result<Message> {
    let (site, excerpt) = pick_excerpt(key)?;
    let message = make_message(site, excerpt);

    Ok(message)
}

pub fn send_one(key: &str, webhook: &str) -> Result<()> {
    let message = get_message(key)?;
    info!("{:?}", message);
    send_message(webhook, message)
}

pub fn main_loop(key: &str, webhook: &str) -> ! {
    loop {
        match send_one(key, webhook) {
            Ok(()) => {
                debug!("sleeping...");
                thread::sleep(Duration::from_secs(60 * 60));
            }
            Err(err) => {
                error!("error: {}", err);
                debug!("sleeping...");
                thread::sleep(Duration::from_secs(5 * 60));
            }
        }
    }
}
