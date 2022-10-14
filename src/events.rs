use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::Ready;
use serenity::prelude::*;

use crate::commands::automod;
use crate::commands::count;
use crate::commands::polls;
use crate::commands::reddit;
use crate::commands::rm;

pub struct Handler {
    pub prefix: &'static str,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let tokens = msg.content.split_whitespace().collect::<Vec<&str>>();

        if !msg.content.starts_with(self.prefix) {
            let is_ok = automod::check_msg(&tokens);
            if let Err(why) = msg.channel_id.say(&ctx.http, is_ok).await {
                println!("Error sending message: {:?}", why);
            }
            return;
        }

        match &tokens[0][1..tokens[0].len()] {
            "ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "rm" => {
                let num = tokens[1].to_string().parse::<i32>().unwrap() as u64;
                rm::rm(ctx, msg, num).await;
            }
            "meme" => {
                let mut subreddit = "dankmemes";
                if tokens.len() > 1 {
                    subreddit = tokens[1];
                }

                let content = reddit::random_meme(subreddit)
                    .await
                    .expect("Reddit api error");
                if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            "poll" => {
                let mut title = String::from("");
                let mut options: Vec<&str> = vec![];
                let mut set_title = false;

                for token in &tokens[1..] {
                    if token.starts_with("\"") && !set_title && title == "" {
                        set_title = true;
                        title += &token[1..]
                    } else if token.ends_with("\"") && set_title {
                        set_title = false;
                        title += " ";
                        title += &token[..token.len() - 1]
                    } else if set_title {
                        title += " ";
                        title += token;
                    }

                    if token.starts_with("\"") && token.ends_with("\"") && !set_title {
                        options.push(&token[1..token.len() - 1]);
                    }
                }

                polls::create_poll(ctx, msg.to_owned(), title.as_str(), options).await;
            }
            "count" => {
                count::create(ctx, msg).await;
            }
            _ => return,
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let msg_interaction = interaction.message_component().unwrap();

        if msg_interaction.data.custom_id != "counter" {
            return;
        }

        msg_interaction
            .create_interaction_response(&ctx, |r| {
                let counter_button = count::get_button();
                let content = msg_interaction.message.content.clone();
                let count: u32 = content.split(":").last().unwrap().trim().parse().unwrap();

                r.kind(InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.content(format!("count: {}", count + 1)).components(|c| {
                            c.create_action_row(|row| row.add_button(counter_button))
                        })
                    })
            })
            .await
            .unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
