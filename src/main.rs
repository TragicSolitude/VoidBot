extern crate serenity;
extern crate toml;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

mod commands;
mod managed_channel;

use serenity::client::Client;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use std::env;
use std::collections::HashMap;
use commands::playing::Playing;
use commands::ping::Ping;
use commands::version::Version;
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

    fn ready(&self, _ctx: Context, data_about: Ready) {
        for guild in data_about.guilds {
            if let Ok(channels) = guild.id().channels() {
                for (_,guild_channel) in channels.iter() {
                    if let Some(topic) = &guild_channel.topic {
                        if topic.contains("$VOIDBOT_DESCRIPTION") {
                            let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/DESCRIPTION.md"));
                            if let Ok(mut messages) = guild_channel.messages(|g| g.limit(100)) {
                                // let _ = guild_channel.delete_messages(messages);
                                let mut edited = false;
                                for message in messages.iter_mut() {
                                    if let Some(line) = message.content.lines().next() {
                                        if line == "#!@" {
                                            let _ = message.edit(|m| m.content(content));
                                            edited = true;
                                            break;
                                        }
                                    }
                                };

                                if !edited {
                                    let _ = guild_channel.say(content);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .cmd("ping", Ping)
        .cmd("playing", Playing)
        .cmd("version", Version));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}