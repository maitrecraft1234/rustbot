use crate::bot::{Context, Error};
use songbird::{input::{Input, YoutubeDl}, tracks::Track};
use crate::utils::reply;

use crate::bot::HttpKey;

/// I PLAY A SONG OR NOT
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_play"
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Optional YouTube URL or search query"] query: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx
        .serenity_context()
        .cache
        .guild(guild_id).ok_or("guild not in cache ?")?
        .voice_states
        .get(&ctx.author().id)
        .and_then(|vs| vs.channel_id).ok_or("not in cache channel ?")?;
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let handler_lock = manager.join(guild_id, channel_id).await.unwrap();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };


    // let track = Track::from(file).volume(1.0);

    let track = if let Some(query) = query {
        // let audio = songbird::input::YoutubeDl::new(http_client, query);
        // let audio = songbird::input::YoutubeDl::new_ytdl_like("yt-dlp", http_client, query);
        let audio = YoutubeDl::new_search(http_client, query);
        let audio = audio.user_args( vec!["-f".to_string(), "bestaudio[ext=m4a]".to_string() ] );
        Track::from(audio)
    } else {
        let file = songbird::input::File::new("./test.m4a"); Track::from(file)
    };

    dbg!(ctx.prefix());
    reply(&ctx, "WE ARE song!!").await?;

    let mut handler = handler_lock.lock().await;
    handler.enqueue(track).await;

    Ok(())
}

fn help_play() -> String {
    "play a song from YouTube".to_string()
}
