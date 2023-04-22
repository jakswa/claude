use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct GreasyHandler {
    pub prompts: Vec<Prompt>,
}

pub struct Prompt {
    prompt: String,
    answers: Vec<String>,
    triggers: Vec<String>,
}

impl GreasyHandler {
    pub fn from_table(table: toml::Table) -> Self {
        let prompts = table["prompts"]
            .as_array()
            .expect("'prompts' should exist in policies.toml")
            .iter()
            .map(|i| Prompt {
                prompt: i["prompt"].as_str().unwrap().to_string(),
                triggers: i["triggers"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|j| j.as_str().unwrap().to_string())
                    .collect(),
                answers: i["answers"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|j| j.as_str().unwrap().to_string())
                    .collect(),
            })
            .collect();
        Self { prompts }
    }
}

#[async_trait]
impl EventHandler for GreasyHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        let prompts = self
            .prompts
            .iter()
            .filter(|i| i.triggers.iter().any(|j| msg.content.contains(j)))
            .collect::<Vec<&Prompt>>();
        if prompts.is_empty() {
            return;
        }
        let prompt = prompts[fastrand::usize(..prompts.len())];
        let answer = prompt.answers[fastrand::usize(..prompt.answers.len())].clone();

        if let Err(why) = msg.channel_id.say(&ctx.http, &prompt.prompt).await {
            println!("Error sending message: {:?}", why);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        if let Err(why) = msg.channel_id.say(&ctx.http, answer).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
