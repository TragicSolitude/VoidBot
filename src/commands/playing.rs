use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use serenity::model::channel::ChannelType;
use crate::CURRENT_SERVER;
use crate::managed_channel::MANAGED_CHANNELS;
use crate::managed_channel::ManagedChannel;

pub struct Playing;

impl Command for Playing {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        // Gets everything after the command
        let name: String = msg.content
            .split(" ")
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" ");
        match CURRENT_SERVER.lock()?.get(&msg.author.id) {
            Some(guild) => {
                if let Some(guild_id) = guild {
                    let new_channel
                        = guild_id.create_channel(&name, ChannelType::Voice, None)?;
                    MANAGED_CHANNELS
                        .lock()?
                        .push(ManagedChannel::new(new_channel.id));

                    guild_id.move_member(msg.author.id, new_channel.id)?;
                }

                Ok(())
            },
            None => {
                msg.reply("You are either not yet tracked or not currently in a voice channel")?;
                Err("No server".into())
            }
        }
    }
}
