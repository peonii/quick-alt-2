use std::sync::Arc;

use twilight_gateway::Event;

mod interaction_create;
mod message_create;
mod ready;

pub async fn handle_event(event: Event, http: Arc<twilight_http::Client>) -> anyhow::Result<()> {
    match event {
        Event::Ready(e) => ready::handler(e),
        Event::MessageCreate(e) => message_create::handler(e, http).await,
        Event::InteractionCreate(e) => interaction_create::handler(e, http).await,
        _ => Ok(()),
    }
}
