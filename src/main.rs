use futures::stream::StreamExt;
use serde_json::Value;
use std::{env, error::Error};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;

pub mod api;
pub mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = include_str!("token.txt").to_owned();

    // This is the default scheme. It will automatically create as many
    // shards as is suggested by Discord.
    let scheme = ShardScheme::Auto;

    // Use intents to only receive guild message events.
    let cluster = Cluster::builder(&token, Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES)
        .shard_scheme(scheme)
        .build()
        .await
        .unwrap();

    // Start up the cluster.
    let cluster_spawn = cluster.clone();

    // Start all shards in the cluster in the background.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // HTTP is separate from the gateway, so create a new client.
    let http = HttpClient::new(&token);

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    let mut events = cluster.events();

    // Process each event as they come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(shard_id, event, http.clone()));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: HttpClient,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            println!("tanklar-handle_event: Got message {}", msg.content);
            match msg.content.as_str() {
                "!help" => {
                    println!("tanklarkit-help: Serving help");
                    let embed = EmbedBuilder::new()
                        .description("Supported commands")
                        .unwrap()
                        .title("Help")
                        .unwrap()
                        .color(0x002f94)
                        .unwrap()
                        .field(
                            EmbedFieldBuilder::new(
                                "!servers",
                                "Display the contents of /readableServers",
                            )
                            .unwrap()
                            .inline(),
                        )
                        .field(
                            EmbedFieldBuilder::new(
                                "!changelog",
                                "Display the contents of /changelogData",
                            )
                            .unwrap()
                            .inline(),
                        )
                        .field(
                            EmbedFieldBuilder::new(
                                "!latest update",
                                "Display the first content of /changelogData",
                            )
                            .unwrap()
                            .inline(),
                        );
                    let embed = embed.build().unwrap();
                    http.create_message(msg.channel_id)
                        .embed(embed)
                        .unwrap()
                        .await
                        .unwrap();
                }
                "!servers" => {
                    println!("tanklarkit-servers: Fetching servers");
                    let v = api::fetch_servers().await;
                    for server in v.content {
                        let mut embed = EmbedBuilder::new()
                            .description("Server Response")
                            .unwrap()
                            .title("Readable Server")
                            .unwrap()
                            .color(0x002f94)
                            .unwrap();
                        embed = embed.field(
                            EmbedFieldBuilder::new("gamemode", server.gameMode)
                                .unwrap()
                                .inline(),
                        );
                        embed = embed.field(
                            EmbedFieldBuilder::new("player count", server.playerCount)
                                .unwrap()
                                .inline(),
                        );
                        embed = embed.field(
                            EmbedFieldBuilder::new("provider", server.provider)
                                .unwrap()
                                .inline(),
                        );
                        embed = embed.field(
                            EmbedFieldBuilder::new("region", server.region)
                                .unwrap()
                                .inline(),
                        );
                        embed = embed.field(
                            EmbedFieldBuilder::new("status", server.status)
                                .unwrap()
                                .inline(),
                        );
                        embed = embed.field(
                            EmbedFieldBuilder::new("permissions", server.permissions.to_string())
                                .unwrap()
                                .inline(),
                        );
                        let embed = embed.build().unwrap();
                        http.create_message(msg.channel_id)
                            .embed(embed)
                            .unwrap()
                            .await
                            .unwrap();
                    }
                }
                "!changelog" => {
                    println!("tanklarkit-changelog: Fetching changelog");
                    let v = api::fetch_changelog().await;
                    for item in v.content {
                        let mut embed = EmbedBuilder::new()
                            .description(item.content)
                            .unwrap()
                            .title(item.title)
                            .unwrap()
                            .color(0x002f94)
                            .unwrap();
                        embed = embed.field(
                            EmbedFieldBuilder::new("version", item.version)
                                .unwrap()
                                .inline(),
                        );
                        let embed = embed.build().unwrap();
                        http.create_message(msg.channel_id)
                            .embed(embed)
                            .unwrap()
                            .await
                            .unwrap();
                    }
                }
                "!latest update" => {
                    println!("tanklarkit-latest_update: Fetching changelog");
                    let mut v = api::fetch_changelog().await;
                    let idx = 0;
                    if idx < v.content.len() {
                        let item = v.content.remove(idx);
                        let mut embed = EmbedBuilder::new()
                            .description(item.content)
                            .unwrap()
                            .title(item.title)
                            .unwrap()
                            .color(0x002f94)
                            .unwrap();
                        embed = embed.field(
                            EmbedFieldBuilder::new("version", item.version)
                                .unwrap()
                                .inline(),
                        );
                        let embed = embed.build().unwrap();
                        http.create_message(msg.channel_id)
                            .embed(embed)
                            .unwrap()
                            .await
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
