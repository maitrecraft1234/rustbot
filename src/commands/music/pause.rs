use crate::bot::{Context, Error};

/// pause the music
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_pause"
)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(track) = handler.queue().current() {
            track.pause()?;
            ctx.reply("⏸ Paused").await?;
        } else {
            ctx.reply("Nothing is playing!").await?;
        }
    }
    Ok(())
}

fn help_pause() -> String {
    "Pause the current track.".to_string()
}
