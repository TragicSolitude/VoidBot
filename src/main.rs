extern crate serenity;
extern crate toml;
extern crate timer;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

mod handler;
mod commands;
mod managed_channel;
mod description;

use serenity::client::Client;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use timer::Timer;
use handler::Handler;

lazy_static! {
    /// Keeps a map of users' current server for easy lookup in the `!playing`
    /// command so that it can determine which server to create the voice
    /// channel in.
    static ref CURRENT_SERVER: Mutex<HashMap<UserId, Option<GuildId>>> = Mutex::new(HashMap::new());
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    
    let time_guard = Arc::new(Mutex::new(Timer::new()));
    
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .cmd("ping", commands::ping::Ping)
        .cmd("playing", commands::playing::Playing)
        .cmd("version", commands::version::Version)
        .cmd("remindme", commands::remindme::RemindMe(time_guard)));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}