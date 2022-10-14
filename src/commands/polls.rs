use serenity::{model::prelude::{Message, ReactionType}, prelude::Context};

pub async fn create_poll(ctx: Context, msg: Message, title: &str, options: Vec<&str>) {
    let mut desc = String::from("");
    let emojis = vec!["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣"];
    
    let mut reactions = vec![];
    for e in &emojis[..options.len()] {
        reactions.push(ReactionType::Unicode(e.to_string()));
    }

    for i in 0..options.len() {
        desc += &format!("{}  {}\n\n", emojis[i], options[i]).to_string(); 
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("new poll for ya suckers")
            .embed(|e| e
                .title(title)
                .description(desc)
            )
            .reactions(reactions)
    }).await.unwrap();
}
