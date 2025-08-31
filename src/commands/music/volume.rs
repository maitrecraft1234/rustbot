use crate::{bot::{Context, Error, SongInfo}, utils::save_song_info};

/// change the volume !
#[poise::command(
    slash_command,
    prefix_command,
    category = "Music",
    help_text_fn = "help_volume"
)]
pub async fn volume(
    ctx: Context<'_>,
    #[description = "Volume between 0.0 and 2.0"] vol: f32,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(track) = handler.queue().current() {
            track.set_volume(vol)?;
            let uuid = track.uuid();
            let name = &ctx.data().song_paths.lock().await[&uuid];
            let mut song_store = ctx.data().song_store.lock().await;
            song_store.insert(name.clone(), SongInfo { volume: vol }).unwrap();
            save_song_info(&song_store);
            ctx.reply(format!("🔊 Volume set to {vol}")).await?;
        } else {
            ctx.reply("Nothing is playing!").await?;
        }
    }
    Ok(())
}

fn help_volume() -> String {
    "Set playback volume (0.0 - 2.0).".to_string()
}
