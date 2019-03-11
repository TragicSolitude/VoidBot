use serenity::prelude::*;
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use serenity::model::guild::GuildStatus;
use crate::description::update_bot_description;
use crate::ChannelManager;

pub struct Handler;

impl EventHandler for Handler {
    fn voice_state_update(&self, ctx: Context, _: Option<GuildId>, state: VoiceState) {
        let mut data = ctx.data.lock();
        data.get_mut::<ChannelManager>().unwrap().voice_state_update(state);
    }

    fn ready(&self, ctx: Context, data_about: Ready) {
        let _ = update_bot_description(&data_about);
        {
            let mut data = ctx.data.lock();
            let channel_manager = data.get_mut::<ChannelManager>().unwrap();

            for guild_status in data_about.guilds {
                if let GuildStatus::OnlineGuild(guild) = guild_status {
                    channel_manager.refresh_voice_states(guild.voice_states);
                }
            }
        }
    }
}
