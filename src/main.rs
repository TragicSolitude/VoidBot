extern crate serenity;
extern crate toml;
extern crate bincode;
extern crate meval;
extern crate rand;
extern crate sentry;
#[macro_use]
extern crate serde;

mod models;
mod handler;
mod commands;
mod channel_manager;
mod description;
mod reminder_manager;
mod error_manager;

use std::env;
use serenity::prelude::*;
use serenity::framework::StandardFramework;
use handler::Handler;
use channel_manager::ChannelManager;
use reminder_manager::ReminderManager;
use error_manager::ErrorManager;

fn main() {
    let event_handler = Handler;
    let mut client =
        Client::new(&env::var("DISCORD_TOKEN").expect("token"), event_handler)
        .expect("Error creating client");

    let channel_manager = ChannelManager::new();
    let reminder_manager = ReminderManager::new();
    let error_manager = ErrorManager::new();
    {
        let mut lock = client.data.lock();
        lock.insert::<ChannelManager>(channel_manager);
        lock.insert::<ReminderManager>(reminder_manager);
        lock.insert::<ErrorManager>(error_manager);
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .on_dispatch_error(|ctx, _, error| {
            let data = ctx.data.lock();
            if let Some(error_manager) = data.get::<ErrorManager>() {
                error_manager.error_log(&format!("{:?}", error));
            }
        })
        .cmd("ping", commands::ping::Ping)
        .cmd("playing", commands::playing::Playing)
        .cmd("allplaying", commands::allplaying::AllPlaying)
        .cmd("version", commands::version::Version)
        .cmd("remindme", commands::remindme::RemindMe)
        .cmd("eval", commands::eval::Eval)
        .cmd("roll", commands::roll::Roll)
        .cmd("teams", commands::teams::Teams));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
