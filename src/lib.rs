use std::{env, sync::Arc};

use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};

pub mod commands;
pub mod events;

pub async fn start() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let token = env::var("BOT_TOKEN")?;

    let intents = Intents::GUILDS
        | Intents::GUILD_MESSAGES
        | Intents::DIRECT_MESSAGES
        | Intents::MESSAGE_CONTENT;

    let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

    let http = Arc::new(twilight_http::Client::new(token));

    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                println!("{}", source.to_string());
                continue;
            }
        };

        cache.update(&event);

        tokio::spawn(events::handle_event(event, Arc::clone(&http)));
    }
}
