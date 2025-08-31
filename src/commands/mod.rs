mod ping;
use ping::ping;

mod help;
use help::help;

mod music;

use crate::bot::{Error, Data};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        help(),
        ping(),
        music::play::play(),
        music::leave::leave(),
        music::skip::skip(),
        music::pause::pause(),
        music::resume::resume(),
        music::volume::volume(),
        music::join::join(),
    ]
}
