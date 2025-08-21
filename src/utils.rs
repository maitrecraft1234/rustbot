use crate::bot::{Context, Error};

pub async fn reply(ctx: &Context<'_>, content: &str) -> Result<(), Error> {
    if ctx.prefix() == "/" {
        ctx.send(poise::CreateReply::default().ephemeral(true).content(content)).await?;
    } else {
        ctx.reply(content).await?;
    }
    Ok(())
}
