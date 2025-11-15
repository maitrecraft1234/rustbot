use crate::utils::{default_activity, reply};
use rand::rng;
use rand::seq::SliceRandom;
use crate::{
    bot::{Context, Error},
    commands::music,
};
use songbird::tracks::{PlayMode, Track};
use poise::serenity_prelude as serenity;
use songbird::{EventHandler as VoiceEventHandler, TrackEvent};
use songbird::{Event, EventContext};
use std::{fs, path::PathBuf};

struct NowPlayingHandler {
    ctx: serenity::Context,
    title: String,
}

pub const VOLUME_REBASE : f32 = 0.1;

#[async_trait::async_trait]
impl VoiceEventHandler for NowPlayingHandler {
    async fn act(
        &self,
        ctx: &EventContext<'_>,
    ) -> Option<Event>
    {
        let ctx_clone = self.ctx.clone();
        let title = self.title.clone();
        let title = title.rsplit_once('/').unwrap_or(("", "how")).1;
        let title = title.rsplit_once('.').unwrap_or(("", "how2")).0;

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

async fn add_folder(ctx: Context<'_>) {
    let mut paths: Vec<PathBuf> = fs::read_dir("./music")
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    paths.shuffle(&mut rng());
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
        let handle = handler.enqueue(track.volume(vol * VOLUME_REBASE)).await;
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
