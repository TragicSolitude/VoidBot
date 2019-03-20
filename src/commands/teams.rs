use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use crate::channel_manager::ChannelManager;
use rand::seq::SliceRandom;

pub struct Teams;

impl Command for Teams {
    fn execute(&self, ctx: &mut Context, msg: &Message, mut args: Args) -> Result<(), Error> {
        let mut data = ctx.data.lock();
        if let Some(channel_manager) = data.get_mut::<ChannelManager>() {
            let num_teams = args.single::<usize>()?;
            let user_location = channel_manager.user_current_channel(&msg.author.id);
            if let Some((guild_id, current_channel_id)) = user_location {
                let mut users_to_move = channel_manager.get_all_users_in_channel(&current_channel_id);
                let num_members = users_to_move.len();
                if num_members < num_teams {
                    let _ = msg.reply("Cannot create more teams than members available to move");
                    return Err("Not enough teams".into());
                }
                let slice = users_to_move.as_mut_slice();
                slice.shuffle(&mut rand::thread_rng());

                let mut i = 1;
                for chunk in slice.chunks(num_members / num_teams) {
                    let new_channel_id = channel_manager
                        .new_managed_channel(&guild_id, &format!("Team {}", i))?;

                    for user_id in chunk {
                        let _ = guild_id.move_member(*user_id, new_channel_id);
                    }

                    i += 1;
                }
            }
        }

        Ok(())
    }
}
