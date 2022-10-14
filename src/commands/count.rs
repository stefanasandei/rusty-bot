use serenity::builder::CreateButton;
use serenity::client::Context;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::Message;

pub fn get_button() -> CreateButton {
    let mut counter_button = CreateButton::default();
    counter_button.label("increment");
    counter_button.style(ButtonStyle::Primary);
    counter_button.custom_id("counter");

    counter_button
}

pub async fn create(ctx: Context, msg: Message) {
    let counter_button = get_button();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content("count: 0")
                .components(|c| c.create_action_row(|row| row.add_button(counter_button)))
        })
        .await
        .unwrap();
}
