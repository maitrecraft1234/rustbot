use crate::utils::{default_activity, reply};
use crate::{
    bot::{Context, Error},
    commands::music,
};
use songbird::tracks::{PlayMode, Track};
use poise::serenity_prelude as serenity;
use songbird::{EventHandler as VoiceEventHandler, TrackEvent};
use songbird::{Event, EventContext};

struct NowPlayingHandler {
    ctx: serenity::Context,
    title: String,
}

#[async_trait::async_trait]
impl VoiceEventHandler for NowPlayingHandler {
    async fn act(
        &self,
        ctx: &EventContext<'_>,
    ) -> Option<Event>
    {
        let ctx_clone = self.ctx.clone();
        let title = self.title.clone();
        let title = title.rsplit_once('/').unwrap().1;
        let title = title.rsplit_once('.').unwrap().0;

        match ctx {
            EventContext::Track(track_list) => {
                for (state, _) in *track_list {
                    if state.playing == PlayMode::Play {
                        ctx_clone.set_presence(
                            Some(serenity::ActivityData::playing(title)),
                            serenity::OnlineStatus::Online,
                        )
                    } else {
                        ctx_clone.set_presence(
                            Some(default_activity()),
                            serenity::OnlineStatus::Online,
                        )
                    }
                }
            }
            _ => {}
        }
        None
    }
}

// does not work well songbird::input::YoutubeDl;
/// I PLAY A SONG OR NOT
#[poise::command(
    slash_command,
    prefix_command,
    aliases("p", "ambiance"),
    category = "Music",
    help_text_fn = "help_play"
)]
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
    music::join::join_internal(ctx).await?;
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
    let mut paths: Vec<PathBuf> = fs::read_dir("./music")
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    shuffle_with_urandom(&mut paths).unwrap();
    for entry in paths {
        add_song(ctx, entry).await
    }
}

async fn add_song(ctx: Context<'_>, path: PathBuf) {
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let guild_id = ctx.guild_id().unwrap();

    let file = songbird::input::File::new(path.clone());
    let track = Track::from(file);

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let path_string = path.into_os_string().into_string().unwrap();
        let vols = ctx.data().song_store.lock().await;
        let vol = vols.get(&path_string).unwrap_or_default().volume;
        ctx.data()
            .song_paths
            .lock()
            .await
            .insert(track.uuid, path_string.clone());
        let handle = handler.enqueue(track.volume(vol)).await;
        handle.add_event(Event::Track(TrackEvent::Play), NowPlayingHandler {
            ctx: ctx.serenity_context().clone(),
            title: path_string,
        }).unwrap();
        handle.add_event(Event::Track(TrackEvent::End), NowPlayingHandler {
            ctx: ctx.serenity_context().clone(),
            title: String::new(),
        }).unwrap();
    }
}

fn help_play() -> String {
    "play a song from the playlist".to_string()
}
