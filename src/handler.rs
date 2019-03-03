use serenity::prelude::*;
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::model::gateway::Ready;
use crate::managed_channel::ManagedChannel;
use crate::description::update_bot_description;
use crate::CURRENT_SERVER;

pub struct Handler;

impl EventHandler for Handler {
    fn voice_state_update(&self, _ctx: Context, guild_id: Option<GuildId>, state: VoiceState) {
        CURRENT_SERVER.lock().insert(state.user_id, guild_id);
        ManagedChannel::recv_update(state.user_id, state.channel_id);
    }

    fn ready(&self, _ctx: Context, data_about: Ready) {
        // TODO Search for new tagged channels whenever someone changes a text
        // channel topic
        let _ = update_bot_description(data_about);
    }
}