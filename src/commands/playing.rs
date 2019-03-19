use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use crate::channel_manager::ChannelManager;

pub struct Playing;

impl Command for Playing {
    fn execute(&self, ctx: &mut Context, msg: &Message, args: Args) -> Result<(), Error> {
        let mut data = ctx.data.lock();
        if let Some(channel_manager) = data.get_mut::<ChannelManager>() {
            let name = args.rest();
            let user_location = channel_manager.user_current_channel(&msg.author.id);
            if let Some((guild_id, _)) = user_location {
                let channel_id = channel_manager.new_managed_channel(&guild_id, name)?;
                let _ = guild_id.move_member(msg.author.id, channel_id);
                return Ok(());
            }
        }

        msg.reply("You are either not yet tracked or not currently in a voice channel")?;
        Err("No server".into())
    }
}
