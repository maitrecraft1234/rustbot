use crate::bot::{Context, Error};
use crate::utils::reply;

// does not work well use songbird::input::YoutubeDl;

/// I JOING THE THING
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_join"
)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    join_internal(ctx).await?;
    dbg!(ctx.prefix());
    reply(&ctx, "WE ARE HERE!!").await
}

pub async fn join_internal(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx
        .serenity_context()
        .cache
        .guild(guild_id)
        .ok_or("guild not in cache ?")?
        .voice_states
        .get(&ctx.author().id)
        .and_then(|vs| vs.channel_id)
        .ok_or("not in cache channel ?")?;
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    manager.join(guild_id, channel_id).await.unwrap();

    Ok(())
}

fn help_join() -> String {
    "the bot joins the channel the user is currently in".to_string()
}
