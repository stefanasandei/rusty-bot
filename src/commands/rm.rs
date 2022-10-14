use serenity::{model::prelude::Message, prelude::Context};

pub async fn rm(ctx: Context, msg: Message, limit: u64) {
    let mut ids = vec![];
    let messages = msg.channel_id
        .messages(&ctx.http, |retriever| retriever.before(msg.id).limit(limit))
        .await.unwrap();

    for m in messages {
        ids.push(m.id);
    }

    msg.channel_id.delete_messages(&ctx.http, ids).await.unwrap();
    msg.delete(&ctx.http).await.unwrap();
}
