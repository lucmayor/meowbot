use std::env;

use serenity::all::Event;
use serenity::{async_trait, client};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // !add [descriptor] by [timestring]
        if msg.content.starts_with("!do") {
            // fill w/ logic
            let user_string = msg.content
                .split(' ')
                .collect();

            // strings past user_string[0], iterate until user_string[n] = "by", rest of string arr for dating
            // for dating, switch on date types (minutes / hours / days / months / years)

            let key_word_position: Result<String, None> = match user_string {
                _ if user_string.contains("by") => user_string.iter().position(|&key| key == "by"),
                _ if user_string.contains("in") => user_string.iter().position(|&key| key == "in"),
                _ => None
            };

            if let Err(err_msg) = msg.channel_id.say(&ctx.http, "placeholder").await {
                println!("Error sending message: {err_msg:?}");
            }
        }
    }

    async fn ready(&self, _:Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
    }
}

// have to specify flavor for whatever reason
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let token = env::var("DISCORD_KEY")
        .expect("Expected token in env.");

    let intents = GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::GUILD_MESSAGES;

    // might have to undo that addressing
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error in client creation.");

    if let Err(err_msg) = client.start().await {
        println!("Client error: {err_msg:?}");
    }
}
