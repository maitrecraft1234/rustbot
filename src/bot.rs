use std::collections::HashMap;

use crate::{commands, events::event_handler, utils::default_activity};
use ollama_rs::Ollama;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
// use ::serenity::all::ActivityData;
use songbird::SerenityInit;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SongInfo {
    pub volume: f32,
}

impl Default for &SongInfo {
    fn default() -> Self {
        &SongInfo {
            volume: 1.0,
        }
    }
}

pub type SongStore = HashMap<String, SongInfo>;

pub struct Data {
    pub ollama: Ollama,
    pub song_store: Mutex<SongStore>,
    pub song_paths: Mutex<HashMap<uuid::Uuid, String>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn start() {
    let token = std::fs::read_to_string(".token")
        .unwrap()
        .trim()
        .to_string();
    let intents = serenity::GatewayIntents::all();
    let ollama = Ollama::default();
    let bytes = std::fs::read(crate::utils::SONG_STORE_PATH).unwrap_or_default();
    let (song_store, _) =
        bincode::serde::decode_from_slice::<SongStore, _>(&bytes, bincode::config::standard())
            .unwrap_or_default();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("p!".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    ollama,
                    song_store: song_store.into(),
                    song_paths: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .activity(default_activity())
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    client.start().await.unwrap();
}
