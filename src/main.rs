extern crate serenity;
#[macro_use]
extern crate lazy_static;

mod commands;
mod managed_channel;

use serenity::client::Client;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use serenity::model::voice::VoiceState;
use std::env;
use std::collections::HashMap;
use commands::playing::Playing;
use commands::ping::Ping;
use managed_channel::ManagedChannel;

lazy_static! {
    static ref CURRENT_SERVER: Mutex<HashMap<UserId, Option<GuildId>>> = Mutex::new(HashMap::new());
}

struct Handler;
impl EventHandler for Handler {
    fn voice_state_update(&self, _ctx: Context, guild_id: Option<GuildId>, state: VoiceState) {
            CURRENT_SERVER.lock().insert(state.user_id, guild_id);

        ManagedChannel::recv_update(state.user_id, state.channel_id);
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .cmd("ping", Ping)
        .cmd("playing", Playing));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}