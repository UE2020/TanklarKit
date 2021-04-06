use crate::model;

pub async fn fetch_servers() -> model::ReadableServersMessage {
    let client = reqwest::Client::new();
    let res = client
        .get("https://tanklar-beta-client.glitch.me/tanklar_api/readableServers")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();
    println!("tanklarkit-fetch_servers: Fetched servers");
    let v: model::ReadableServersMessage = serde_json::from_str(res.as_str()).unwrap();
    v
}

pub async fn fetch_changelog() -> model::ChangelogMessage {
    let client = reqwest::Client::new();
    let res = client
        .get("https://tanklar-beta-client.glitch.me/tanklar_api/changelogData")
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();
    println!("tanklarkit-fetch_changelog: Fetched servers");
    let v: model::ChangelogMessage = serde_json::from_str(res.as_str()).unwrap();
    v
}
