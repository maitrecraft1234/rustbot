use crate::bot::{Error, Context};

/// resume the song
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_resume"
)]
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(track) = handler.queue().current() {
            track.play()?;
            ctx.reply("▶️ Resumed").await?;
        } else {
            ctx.reply("Nothing is paused!").await?;
        }
    }
    Ok(())
}

fn help_resume() -> String {
    "Resume playback if paused.".to_string()
}
