use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReadableServersMessage {
    pub content: Vec<ReadableServersContent>,
    pub contentType: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReadableServersContent {
    pub id: String,
    pub gameMode: String,
    pub permissions: u8,
    pub playerCount: String,
    pub provider: String,
    pub region: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangelogMessage {
    pub content: Vec<ChangelogContent>,
    pub contentType: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangelogContent {
    pub content: String,
    pub date: String,
    pub title: String,
    pub version: String,
    pub warn: bool,
}
