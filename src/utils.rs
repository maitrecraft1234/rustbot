use std::{fs::File, io::{BufWriter, Write}};

use serenity::all::ActivityData;

use crate::bot::{Context, Error, SongStore};

pub const SONG_STORE_PATH: &'static str = ".song_store";

pub async fn reply(ctx: &Context<'_>, content: &str) -> Result<(), Error> {
    if ctx.prefix() == "/" {
        ctx.send(poise::CreateReply::default().ephemeral(true).content(content)).await?;
    } else {
        ctx.reply(content).await?;
    }
    Ok(())
}

pub fn save_song_info(song_store: &SongStore) {
    let file = File::create(SONG_STORE_PATH).expect("IO ERROR EVEYRTHING EXPLODES");
    let mut writer = BufWriter::new(file);
    bincode::serde::encode_into_std_write(song_store, &mut writer, bincode::config::standard())
        .expect("Serialization failed");
    writer.flush().unwrap();
}

pub fn default_activity() -> ActivityData {
    ActivityData::competing("china")
}

