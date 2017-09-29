#[macro_use]
extern crate serenity;

mod commands;

use serenity::framework::standard::StandardFramework;
use serenity::model::*;
use serenity::prelude::*;
use std::env;

struct Handler;
impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn on_guild_member_addition(&self, _: Context, guild_id: GuildId, member: Member) {
        &guild_id.as_channel_id().say(&format!("{} welcome to the unofficial Cuberite Discord server!", &member.mention()));
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c
            .on_mention(true)
            .prefix("c."))
            .command("latency", |c| c.exec(commands::core::latency))
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
