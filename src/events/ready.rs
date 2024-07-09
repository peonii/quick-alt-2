use tracing::info;
use twilight_model::gateway::payload::incoming::Ready;

pub fn handler(event: Box<Ready>) -> anyhow::Result<()> {
    info!(
        "Logged in as {}#{} successfully.",
        event.user.name,
        event.user.discriminator()
    );

    Ok(())
}
