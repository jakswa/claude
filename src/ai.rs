use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

const CLAUDE_MODEL: &'static str = "claude-3-haiku-20240307";

pub struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Ok(false) = msg.mentions_me(&ctx).await {
            return;
        }

        let ask_ai = regex::Regex::new(r"> (?P<prompt>.*)").unwrap();
        let prompt = ask_ai
            .captures_iter(&msg.content)
            .find(|capture| capture.name("prompt").is_some());
        if let None = prompt {
            return;
        }
        let response = anthropic_prompt(prompt.unwrap().name("prompt").unwrap().as_str());
        msg.reply(&ctx.http, response.await)
            .await
            .expect("hope this works");
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!(":: AI plugin is connected as {}.", ready.user.name);
    }
}

async fn anthropic_prompt(prompt: &str) -> String {
    let client = reqwest::Client::new();
    let api_key = std::env::var("CLAUDE_KEY").expect("set CLAUDE_KEY you dolt");
    let prompt_message = json::object! {
        role: "user",
        content: prompt
    };
    let body = json::object! {
        model: CLAUDE_MODEL,
        max_tokens: 60,
        system: "Try to keep responses about the size of a tweet on twitter, 140 characters.",
        messages: [prompt_message]
    };
    let message: AnthropicMessage = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .body(body.dump())
        .send()
        .await
        .expect("well shit")
        .json()
        .await
        .expect("oh well shit");

    message
        .content
        .expect("should be some content")
        .into_iter()
        .find(|i| i.text.is_some())
        .unwrap()
        .text
        .unwrap()
        .clone()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AnthropicMessage {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub role: Option<String>,
    pub content: Option<Vec<AnthropicContentBlock>>,
    pub model: Option<String>,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    pub usage: Option<AnthropicUsage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnthropicUsage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}
