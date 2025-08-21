use poise::serenity_prelude::{self as serenity, FullEvent};
use crate::bot::{Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase().ends_with("quoi") {
                new_message.reply(&ctx.http, "feur").await?;
            }
        }
        _ => {}
    }
    Ok(())
}
