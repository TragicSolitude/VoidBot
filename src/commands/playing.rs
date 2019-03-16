use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use crate::channel_manager::ChannelManager;

pub struct Playing;

impl Command for Playing {
    fn execute(&self, ctx: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        let mut data = ctx.data.lock();
        if let Some(channel_manager) = data.get_mut::<ChannelManager>() {
            // Gets everything after the command
            let name: String = msg.content
                .split(" ")
                .skip(1)
                .collect::<Vec<&str>>()
                .join(" ");
            if let Some(guild_id) = channel_manager.user_active_guild(&msg.author.id) {
                let channel_id = channel_manager.new_managed_channel(&guild_id, &name)?;
                let _ = guild_id.move_member(msg.author.id, channel_id);
                return Ok(());
            }
        }

        msg.reply("You are either not yet tracked or not currently in a voice channel")?;
        Err("No server".into())
    }
}
