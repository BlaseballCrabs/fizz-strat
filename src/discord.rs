use anyhow::Result;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub title: String,
    pub description: String,
    pub url: String,
    pub timestamp: DateTime<Utc>,
    pub author: EmbedAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub embeds: Vec<Embed>,
}

pub fn send_message(webhook: &str, message: Message) -> Result<()> {
    attohttpc::post(webhook)
        .json(&message)?
        .send()?
        .error_for_status()?;

    Ok(())
}
