mod bot;
mod commands;
mod events;
mod utils;
mod ollama;

#[tokio::main]
async fn main() {
    bot::start().await;
}
