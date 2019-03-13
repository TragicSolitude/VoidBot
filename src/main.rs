extern crate serenity;
extern crate toml;
extern crate bincode;
extern crate regex;
extern crate meval;
#[macro_use]
extern crate serde;

mod models;
mod handler;
mod commands;
mod channel_manager;
mod description;
mod reminder_manager;

use std::env;
use serenity::prelude::*;
use serenity::framework::StandardFramework;
use handler::Handler;
use channel_manager::ChannelManager;
use reminder_manager::ReminderManager;

fn main() {
    let event_handler = Handler;
    let mut client =
        Client::new(&env::var("DISCORD_TOKEN").expect("token"), event_handler)
        .expect("Error creating client");

    let channel_manager = ChannelManager::new();
    let reminder_manager = ReminderManager::new();
    {
        let mut lock = client.data.lock();
        lock.insert::<ChannelManager>(channel_manager);
        lock.insert::<ReminderManager>(reminder_manager);
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .cmd("ping", commands::ping::Ping)
        .cmd("playing", commands::playing::Playing)
        .cmd("version", commands::version::Version)
        .cmd("remindme", commands::remindme::RemindMe)
        .cmd("eval", commands::eval::Eval));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
