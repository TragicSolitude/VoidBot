use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::thread;
use std::time::Duration;
use serenity::model::id::UserId;
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::model::voice::VoiceState;
use serenity::model::channel::ChannelType;
use serenity::Error;
use serenity::prelude::*;
use crate::models::managed_channel::ManagedChannel;

/// Tracks user voice states in order to maintain a list of "temporary" channels
/// that are to be deleted by the bot when the last person leaves
pub struct ChannelManager {
    managed_channels: Arc<Mutex<Vec<ManagedChannel>>>,
    voice_states: HashMap<UserId, VoiceState>
}

impl TypeMapKey for ChannelManager {
    type Value = Self;
}

impl ChannelManager {
    pub fn new() -> Self {
        let managed_channels =
            match File::open("persistence/managed_channels.bin") {
                Ok(mut f) => {
                    let mut bin = Vec::new();
                    let _ = f.read_to_end(&mut bin);
                    match bincode::deserialize::<Vec<ManagedChannel>>(&bin) {
                        Ok(heap) => Arc::new(Mutex::new(heap)),
                        Err(_) => Arc::new(Mutex::new(Vec::new()))
                    }
                }
                Err(_) => Arc::new(Mutex::new(Vec::new()))
            };

        {
            let managed_channels = managed_channels.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(10));
                    if let Ok(lock) = managed_channels.lock() {
                        if let Ok(bin) = bincode::serialize(&*lock) {
                            if let Ok(mut f) = File::create("persistence/managed_channels.bin") {
                                let _ = f.write(&bin);
                            }
                        }
                    }
                }
            });
        }

        ChannelManager {
            managed_channels,
            voice_states: HashMap::new()
        }
    }

    pub fn new_managed_channel(&mut self, guild_id: &GuildId, name: &str) -> Result<ChannelId, Error> {
        let new_channel =
            guild_id.create_channel(name, ChannelType::Voice, None)?;
        match self.managed_channels.lock() {
            Ok(mut lock) => lock.push(ManagedChannel::from(new_channel.id)),
            Err(_) => return Err(Error::Other(""))
        }

        Ok(new_channel.id)
    }

    pub fn user_current_channel(&self, user: &UserId) -> Option<(GuildId, ChannelId)> {
        let voice_state = self.voice_states.get(user)?;
        let channel_id = voice_state.channel_id?;
        if let Ok(channel) = channel_id.to_channel() {
            return Some((channel.guild()?.read().guild_id, channel_id));
        }

        None
    }

    pub fn get_all_users_in_channel<'a>(&'a self, channel_id: &'a ChannelId) -> Vec<&'a UserId> {
        self.voice_states
            .iter()
            .filter(|item| item.1.channel_id == Some(*channel_id))
            .map(|item| item.0)
            .collect()
    }

    pub fn refresh_voice_states(&mut self, new_states: HashMap<UserId, VoiceState>) {
        self.voice_states = new_states;
        self.prune_channels();
    }

    pub fn voice_state_update(&mut self, new_state: VoiceState) {
        self.voice_states.insert(new_state.user_id, new_state);
        self.prune_channels();
    }

    fn prune_channels(&mut self) {
        let states = self.voice_states.values().collect::<Vec<&VoiceState>>();
        if let Ok(mut lock) = self.managed_channels.lock() {
            lock.retain(|channel| {
                for voice_state in states.iter() {
                    if voice_state.channel_id == Some(channel.channel_id()) {
                        return true;
                    }
                }

                false
            });
        }
    }
}
