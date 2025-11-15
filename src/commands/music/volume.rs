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
    #[description = "Volume expressed as a positive float."] vol: f32,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if let Some(track) = handler.queue().current() {
            track.set_volume(vol)?;
            let uuid = track.uuid();
            let name = &ctx.data().song_paths.lock().await[&uuid];
            let mut volumes = ctx.data().song_store.lock().await;
            let current_song_volume = volumes.get(name).unwrap_or_default().volume;
            let newvol = vol * current_song_volume;
            let _ = volumes.insert(name.clone(), SongInfo { volume: newvol});
            save_song_info(&volumes);
            ctx.reply(format!("🔊 Volume set to {newvol}")).await?;
        } else {
            ctx.reply("Nothing is playing!").await?;
        }
    }
    Ok(())
}

fn help_volume() -> String {
    "Set playback volume.".to_string()
}
