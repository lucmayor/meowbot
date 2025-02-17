use std::{env, string};

use serenity::all::Event;
use serenity::{async_trait, client};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use regex::Regex;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // change this to be dict
        let months = [
            "jan", "feb", "mar",
            "apr", "may", "jun",
            "jul", "aug", "sept",
            "oct", "nov", "dec",
            "january", "february", "march",
            "april", "june", "july", "august",
            "september", "october", "november", "december"
        ];

        // !add [descriptor] by [timestring]
        if msg.content.starts_with("!do") {
            // fill w/ logic
            let user_string = msg.content;


            // note, replace all this with regex: (.+?)\s+(by|in)\s+(.+)

            // strings past user_string[0], iterate until user_string[n] = "by", rest of string arr for dating
            // for dating, switch on date types (minutes / hours / days / months / years)
            
            // by case -> (specific date) / ('next' 'weekdate') .. we can have reminder time be stock for any given user / we have not 
            // in case -> (timeslot) (measurement)

            let mut casing: Option<&str> = None;

            let string_tuple: (String, String) = match user_string {
                _ if user_string.contains("by") => user_string.find(" by ")
                                                    .map(|pos| {
                                                        let(before, after) = user_string.split_at(pos);
                                                        let date_part = &after[4..];
                                                        casing = Some("by");
                                                        (before.trim().to_string(), date_part.trim().to_string())
                                                    }),
                _ if user_string.contains("in") => user_string.find(" in ")
                                                    .map(|pos| {
                                                        let(before, after) = user_string.split_at(pos);
                                                        let date_part = &after[4..];
                                                        casing = Some("in");
                                                        (before.trim().to_string(), date_part.trim().to_string())
                                                    }),
                _ => None
            }.unwrap();

            // change these later, needed 

            // old statement for thing
            // let date_regex: Option<Regex> = match casing.unwrap() {
            //     "by" => Some(Regex::new("").unwrap()),
            //     "in" => Some(Regex::new("").unwrap()),
            //     _ => None
            // };
            // let date_fix = date_regex.unwrap().captures(&string_tuple.1) else {
            //     println!("Error with value for date case");
            //     return;
            // };

            let date_fix: Option<i64> = match &string_tuple.1.chars().filter(|c| *c == ' ').count() {
                0 => Some(1), // temp
                1 => match &string_tuple.1 {
                    _ if &string_tuple.1.contains("the") => 1,
                    _ if &string_tuple.1.contains("next") => 1,
                    _ if months.iter().any(|&m| m == &string_tuple.1.split_whitespace().next()) => 1
                }
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
