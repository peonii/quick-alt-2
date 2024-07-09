use std::sync::Arc;

use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handler(msg: Box<MessageCreate>, http: Arc<Client>) -> anyhow::Result<()> {
    if msg.content == "quick alt" {
        http.create_message(msg.channel_id)
            .content("that's me")?
            .await?;
    }

    Ok(())
}
