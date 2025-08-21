use crate::bot::{Context, Error};

/// Leave the voice channel
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_leave"
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Must be in a guild")?;
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
        ctx.say("Left voice channel").await?;
    } else {
        ctx.say("Not in a voice channel").await?;
    }
    Ok(())
}

fn help_leave() -> String {
    "Leave the voice channel idk what else to say".to_string()
}
