use crate::bot::{Context, Error};

/// I REPLY WITH PONG FUCKER
#[poise::command(
    slash_command,
    prefix_command,
    category = "General",
    help_text_fn = "help_ping",
)]
pub async fn ping(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}

fn help_ping() -> String {
    "Ping the bot and whatnot".to_string()
}
