mod ping;
use ping::ping;

mod play;
use play::play;

mod leave;
use leave::leave;

mod help;
use help::help;

use crate::bot::{Error, Data};

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping(),
        play(),
        leave(),
        help(),
    ]
}
