#[macro_use]
extern crate serenity;

mod commands;

use serenity::Client;
use serenity::model::Mentionable;
use std::env;

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").unwrap());

    client.with_framework(|f| f
        .configure(|c| c
            .prefix("c."))
        .command("latency", |c| c.exec(commands::core::latency))
    );

    client.on_guild_member_add(|_ctx, guild_id, member| {
        &guild_id.as_channel_id().say(&format!("{} welcome to the unofficial Cuberite Discord server!", &member.mention()));
    });

    client.on_ready(|_ctx, ready| {
        println!("{} is connected!", ready.user.name);
    });

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
