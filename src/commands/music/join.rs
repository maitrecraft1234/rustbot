use crate::bot::{Context, Error};
use crate::utils::reply;
use poise::serenity_prelude as serenity;

// does not work well use songbird::input::YoutubeDl;

/// I JOING THE THING
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_join"
)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    match join_internal(ctx).await {
        Ok(()) => reply(&ctx, "WE ARE HERE!!").await,
        Err(error) => {
            eprintln!("Failed to join voice channel: {error}");
            reply(&ctx, &format!("Could not join your voice channel: {error}")).await
        }
    }
}

pub async fn join_internal(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("This command must be used in a server")?;
    let channel_id = ctx
        .serenity_context()
        .cache
        .guild(guild_id)
        .ok_or("guild not in cache ?")?
        .voice_states
        .get(&ctx.author().id)
        .and_then(|vs| vs.channel_id)
        .ok_or("not in cache channel ?")?;

    let serenity_ctx = ctx.serenity_context();
    let bot_id = serenity_ctx.cache.current_user().id;
    let bot_member = guild_id.member(serenity_ctx, bot_id).await?;
    let guild = guild_id.to_partial_guild(serenity_ctx).await?;
    let channel = channel_id
        .to_channel(serenity_ctx)
        .await?
        .guild()
        .ok_or("The selected channel is not a server voice channel")?;
    let permissions = guild.user_permissions_in(&channel, &bot_member);
    let connected_users = serenity_ctx
        .cache
        .guild(guild_id)
        .map(|guild| {
            guild
                .voice_states
                .values()
                .filter(|state| state.channel_id == Some(channel_id))
                .count()
        })
        .unwrap_or_default();
    let missing_permissions: Vec<&str> = [
        (serenity::Permissions::VIEW_CHANNEL, "View Channel"),
        (serenity::Permissions::CONNECT, "Connect"),
        (serenity::Permissions::SPEAK, "Speak"),
    ]
    .into_iter()
    .filter_map(|(permission, name)| (!permissions.contains(permission)).then_some(name))
    .collect();

    if !missing_permissions.is_empty() {
        return Err(std::io::Error::other(format!(
            "the bot is missing these permissions in your voice channel: {}",
            missing_permissions.join(", ")
        ))
        .into());
    }

    if channel.user_limit.is_some_and(|limit| {
        limit > 0
            && connected_users >= limit as usize
            && !permissions.contains(serenity::Permissions::MOVE_MEMBERS)
    }) {
        return Err(std::io::Error::other(
            "the voice channel is full and the bot does not have Move Members permission",
        )
        .into());
    }

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or("Songbird was not initialized")?
        .clone();

    manager.join(guild_id, channel_id).await?;

    Ok(())
}

fn help_join() -> String {
    "the bot joins the channel the user is currently in".to_string()
}
