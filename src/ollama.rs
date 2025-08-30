use std::sync::Arc;

use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use serenity::all::{CacheHttp, Http, Message};
use serenity::client::Context;

const MODEL: &str = "gemma2:2b";

pub async fn ollama_generate(ollama: &Ollama, prompt: &str) -> String {
    println!("Generating with ollama with prompt: {prompt}");
    let res = ollama
        .generate(GenerationRequest::new(MODEL.to_string(), prompt))
        .await
        .unwrap()
        .response;
    println!("ollama response: {res}");
    res
}

pub async fn prompt_from_message(cachehttp: impl CacheHttp, message: &Message) -> String {
    let mut prompt = format!(
        "user: '{}' sent '{}'",
        message
            .author_nick(&cachehttp)
            .await
            .unwrap_or(String::from("unkown")),
        message.content
    );
    println!("Initial prompt: {prompt}");
    let mut current = message.referenced_message.as_deref();
    while let Some(reply) = current {
        prompt = format!(
            "{} replying to user: '{}' sent '{}'",
            prompt,
            reply
            .author_nick(&cachehttp)
            .await
            .unwrap_or_else(|| "unknown".into()),
            reply.content,
        );
        current = reply.referenced_message.as_deref();
    }

    prompt
}
