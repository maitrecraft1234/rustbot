mod bot;
mod commands;
mod events;
mod utils;

#[tokio::main]
async fn main() {
    bot::start().await;
}
