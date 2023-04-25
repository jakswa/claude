use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use regex::Regex;

pub struct GreasyHandler {
    pub prompts: Vec<Prompt>,
}

pub struct Prompt {
    prompt: String,
    answers: Vec<String>,
    defaults: toml::Table,
    triggers: Vec<regex::Regex>,
}

impl GreasyHandler {
    pub fn from_table(table: toml::Table) -> Self {
        let prompts = table["prompts"]
            .as_array()
            .expect("'prompts' should exist in policies.toml")
            .iter()
            .map(|i| Prompt {
                prompt: i["prompt"].as_str().expect("'prompt' value is required on each prompt").to_string(),
                triggers: i["triggers"]
                    .as_array()
                    .expect("'triggers' are required in each prompt")
                    .iter()
                    .map(|j| {
                        let mut s = j.as_str().unwrap().to_string();
                        let m = Regex::new(r"<(?P<var>\w+)>").unwrap();
                        m.captures_iter(&s.clone()).for_each(|cg| {
                            let var_name = cg.name("var").unwrap().as_str();
                            s = s.replace(&format!("<{}>", &var_name), &format!(r"(?P<{}><@\w+>)", &var_name));
                        });
                        //
                        Regex::new(&s).unwrap()
                    })
                    .collect(),
                defaults: i.get("defaults").unwrap_or(&toml::Table::new().into()).as_table().unwrap().clone(),
                answers: i.get("answers")
                    .unwrap_or(&toml::value::Array::new().into())
                    .as_array()
                    .unwrap_or(&vec![])
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
        match msg.mentions_me(&ctx).await {
            Ok(false) => return,
            _ => {}
        }
        let prompts = self
            .prompts
            .iter()
            .filter(|i| i.triggers.iter().any(|j| j.is_match(&msg.content)))
            .collect::<Vec<&Prompt>>();
        if prompts.is_empty() {
            return;
        }
        let prompt = prompts[fastrand::usize(..prompts.len())];

        let mut proompt = prompt.prompt.clone();

        prompt.defaults.iter().for_each(|(k,v)| {
            if let Some(m) = prompt.triggers.iter().find_map(|i| i.captures(&msg.content)) {
                if let Some(vv) = m.name(&k) {
                    proompt = proompt.replace(&format!("<{}>", &k), vv.as_str());
                }
            }
            proompt = proompt.replace(&format!("<{}>", &k), v.as_str().expect("default values should be strings"));
        });

        proompt = proompt.replace("<author>", &msg.author.mention().to_string());

        //let response = format!("sure {}, {}", msg.author.mention(), proompt); 
        if let Err(why) = msg.reply(&ctx.http, proompt).await {
            println!("Error sending message: {:?}", why);
        }

        if prompt.answers.is_empty() { return; }

        let answer = prompt.answers[fastrand::usize(..prompt.answers.len())].clone();
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        if let Err(why) = msg.channel_id.say(&ctx.http, answer).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
