extern crate serenity;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

mod models;
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
use handler::Handler;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::collections::BinaryHeap;
use models::reminder::Reminder;

lazy_static! {
    /// Keeps a map of users' current server for easy lookup in the `!playing`
    /// command so that it can determine which server to create the voice
    /// channel in.
    static ref CURRENT_SERVER: Mutex<HashMap<UserId, Option<GuildId>>>
        = Mutex::new(HashMap::new());
    /// Binary heap tracking pending reminders
    static ref REMINDERS: Arc<Mutex<BinaryHeap<Reminder>>>
        = Arc::new(Mutex::new(BinaryHeap::new()));
}

fn main() {
    let mut threads = Vec::new();

    threads.push(thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut lock = REMINDERS.lock();
            while lock.len() > 0 && lock.peek().unwrap().expiration < Instant::now() {
                let reminder = lock.pop().unwrap();
                let _ = reminder.channel.say(
                    format!("{}: {}", reminder.who.mention(), reminder.message));
            }
        }
    }));

    threads.push(thread::spawn(|| {
        let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
            .expect("Error creating client");
        
        client.with_framework(StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .cmd("ping", commands::ping::Ping)
            .cmd("playing", commands::playing::Playing)
            .cmd("version", commands::version::Version)
            .cmd("remindme", commands::remindme::RemindMe));

        // start listening for events by starting a single shard
        if let Err(why) = client.start() {
            println!("An error occurred while running the client: {:?}", why);
        }
    }));

    for i in threads {
        let _ = i.join();
    }
}