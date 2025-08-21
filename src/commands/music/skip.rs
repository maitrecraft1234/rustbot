use crate::bot::{Error, Context};

/// skip the mustic playing
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_skip"
)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.queue().skip().is_ok() {
            ctx.reply("⏭ Skipped!").await?;
        } else {
            ctx.reply("Nothing to skip!").await?;
        }
    }
    Ok(())
}

fn help_skip() -> String {
    "Skip the currently playing track.\nUsage: /skip".to_string()
}
