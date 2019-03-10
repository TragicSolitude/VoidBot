extern crate serenity;
extern crate toml;
extern crate bincode;
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
use serenity::model::id::ChannelId;
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use handler::Handler;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::collections::BinaryHeap;
use models::reminder::Reminder;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    /// Keeps a map of users' current server for easy lookup in the `!playing`
    /// command so that it can determine which server to create the voice
    /// channel in.
    static ref CURRENT_SERVER: Arc<Mutex<HashMap<UserId, Option<GuildId>>>>
        = Arc::new(Mutex::new(HashMap::new()));
    /// Binary heap tracking pending reminders, backed by "reminders.bin" file
    /// for persistence
    static ref REMINDERS: Arc<Mutex<BinaryHeap<Reminder>>> = {
        // TODO make wrapper class that handles all of this stuff for us
        if let Ok(mut f) = File::open("persistence/reminders.bin") {
            let mut bin = Vec::new();
            let _ = f.read_to_end(&mut bin);
            if let Ok(heap) = bincode::deserialize::<BinaryHeap<Reminder>>(&bin) {
                println!("Loaded {} reminders from file", heap.len());
                return Arc::new(Mutex::new(heap));
            }
        }

        println!("Creating new reminders queue");
        Arc::new(Mutex::new(BinaryHeap::new()))
    };
}

fn main() {
    let mut threads = Vec::new();

    threads.push(thread::spawn(|| {
        // TODO move this stuff into a self-contained struct
        loop {
            thread::sleep(Duration::from_secs(10));
            if let Ok(lock) = REMINDERS.lock() {
                if let Ok(bin) = bincode::serialize(&*lock) {
                    if let Ok(mut f) = File::create("persistence/reminders.bin") {
                        let _ = f.write(&bin);
                    }
                }
            }
        }
    }));

    threads.push(thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            if let Ok(mut lock) = REMINDERS.lock() {
                while lock.len() > 0 && lock.peek().unwrap().expiration < SystemTime::now() {
                    let reminder = lock.pop().unwrap();
                    let _ = ChannelId::from(reminder.channel).say(
                        format!("{}: {}", UserId::from(reminder.author).mention(), reminder.message));
                }
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
