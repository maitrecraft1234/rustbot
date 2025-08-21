use crate::bot::{Context, Error};
use songbird::{tracks::Track, EventContext, TrackEvent};
use crate::utils::reply;

// does not work well use songbird::input::YoutubeDl;

/// I PLAY A SONG OR NOT
#[poise::command(
    slash_command,
    prefix_command,
    aliases("p", "ambiance"),
    category = "Music",
    help_text_fn = "help_play"
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Optional YouTube URL or search query"] query: Option<String>,
) -> Result<(), Error> {

    dbg!(ctx.prefix());
    reply(&ctx, "WE ARE song!!").await?;

    add_folder(ctx).await;

    Ok(())
}
use std::{fs, io::Read, path::PathBuf};

fn shuffle_with_urandom(vec: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let mut urandom = fs::File::open("/dev/urandom")?;
    let mut buf = [0u8; 8];

    for i in (1..vec.len()).rev() {
        urandom.read_exact(&mut buf)?;
        let r = u64::from_ne_bytes(buf) as usize;
        let j = r % (i + 1);
        vec.swap(i, j);
    }

    Ok(())
}

async fn add_folder(ctx: Context<'_>) {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    let mut paths: Vec<PathBuf> = fs::read_dir("./music")
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    shuffle_with_urandom(&mut paths).unwrap();
    for entry in paths {
        let entry = entry;
        let file = songbird::input::File::new(entry);
        let track = Track::from(file);
        if let Some(handler_lock) = manager.get(guild_id) {
            let mut handler = handler_lock.lock().await;
            handler.enqueue(track).await;
        }
    }
}

fn help_play() -> String {
    "play a song from the playlist".to_string()
}
