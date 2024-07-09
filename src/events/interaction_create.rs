use std::sync::Arc;

use twilight_http::Client;
use twilight_model::{
    application::interaction::{InteractionData, InteractionType},
    gateway::payload::incoming::InteractionCreate,
};

pub async fn handler(interaction: Box<InteractionCreate>, http: Arc<Client>) -> anyhow::Result<()> {
    match interaction.kind {
        InteractionType::ApplicationCommand => {
            if let Some(InteractionData::ApplicationCommand(data)) = interaction.data.as_ref() {}
        }
        _ => {}
    }

    Ok(())
}
