use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serde::Deserialize;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Deserialize, Default)]
pub struct Response {
    text: Option<String>,
    choices: Option<Vec<String>>,
    delay: Option<std::time::Duration>,
    is_reply: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    #[serde(with = "serde_regex")]
    matches: Vec<Regex>,
    responses: Vec<Response>,
    defaults: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Handler {
    pub commands: Vec<Command>,
}

impl Handler {
    pub fn from_str(str: &str) -> Self {
        let mut str_copy = str.to_string();
        let match_vars = Regex::new(r"<<(?P<var>\w+)>>").unwrap();

        // replacing special <<var>>s that are only used in match strings.
        // this lets users not need to know the regex syntax to capture the var.
        match_vars.captures_iter(&str).for_each(|var_capture| {
            let var_name = var_capture.name("var").unwrap().as_str();
            str_copy = str_copy.replace(
                &format!("<<{}>>", var_name),
                &format!(r"(?P<{}><@\w+>)", var_name),
            );
        });

        toml::from_str(&str_copy).expect("failed to parse")
    }

    fn sub_variables(cmd: &Command, txt: &str, msg: &Message) -> String {
        let mut response_text = txt.to_string();

        if let Some(defaults) = &cmd.defaults {
            defaults.iter().for_each(|(k, v)| {
                if let Some(m) = cmd.matches.iter().find_map(|i| i.captures(&msg.content)) {
                    if let Some(vv) = m.name(k) {
                        response_text = response_text.replace(&format!("<{}>", &k), vv.as_str());
                    }
                }
                response_text = response_text.replace(&format!("<{}>", &k), v);
            });
        }

        response_text.replace("<author>", &msg.author.mention().to_string())
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Ok(false) = msg.mentions_me(&ctx).await {
            return;
        }
        let commands = self
            .commands
            .iter()
            .filter(|i| i.matches.iter().any(|j| j.is_match(&msg.content)))
            .collect::<Vec<&Command>>();
        if commands.is_empty() {
            let fallback = crate::ai::Handler {};
            fallback.message(ctx, msg).await;
            return;
        }
        let command = commands[fastrand::usize(..commands.len())];
        for response in command.responses.iter() {
            if let Some(delay) = response.delay {
                tokio::time::sleep(delay).await;
            }

            let mut response_text = response
                .text
                .clone()
                .or(response
                    .choices
                    .clone()
                    .map(|i| i[fastrand::usize(..i.len())].clone()))
                .expect("each response needs either 'text' or an array of 'choices'");
            response_text = Handler::sub_variables(command, &response_text, &msg);

            let reply = match response.is_reply.unwrap_or(false) {
                true => msg.reply(&ctx.http, response_text).await,
                false => msg.channel_id.say(&ctx.http, response_text).await,
            };

            if let Err(why) = reply {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            ":: Commands TOML plugin is connected as {}.",
            ready.user.name
        );
    }
}
