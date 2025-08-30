use poise::serenity_prelude::{self as serenity, FullEvent};
use crate::{bot::{Data, Error}, ollama::{ollama_generate, prompt_from_message}};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().ends_with("quoi") {
                new_message.reply(&ctx.http, "feur").await?;
            }
            if new_message.mentions.iter().any(|u| u.id == ctx.cache.current_user().id) {
                let prompt = prompt_from_message(&ctx, &new_message).await;
                let res = ollama_generate(&data.ollama, &prompt).await; 
                new_message.reply(&ctx.http, res).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
