use serenity::model::id::UserId;
use serenity::model::id::ChannelId;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    /// Contains a list of temporary channels currently being managed by the
    /// bot.
    pub static ref MANAGED_CHANNELS: Mutex<Vec<ManagedChannel>> = Mutex::new(Vec::new());
}

/// Tracks channels that were created by the bot using the `!playing` command
/// along with the necessary stats required to efficiently check if the channel
/// needs to be pruned.
/// 
/// ManagedChannel implements the `Drop` trait so that the channel being managed
/// by a particular instance is automatically deleted when the instance goes out
/// of scope.
pub struct ManagedChannel {
    pub id: ChannelId,
    pub users: HashSet<UserId>
}

impl Drop for ManagedChannel {
    fn drop(&mut self) {
        // Delete associated channel when instance goes out of scope
        let _ = self.id.delete();
    }
}

impl ManagedChannel {
    pub fn new(id: ChannelId) -> Self {
        ManagedChannel {
            id,
            users: HashSet::new()
        }
    }

    /// Handles pruning of empty managed channels by tracking user movements and
    /// keeping the MANAGED_CHANNELS static variable up to date with the current
    /// status of each managed channel.
    /// 
    /// This should be called by the `voice_state_update` event handler.
    pub fn recv_update(user: UserId, new_channel: Option<ChannelId>) {
        if let Ok(mut managed_channels) = MANAGED_CHANNELS.lock() {
            for channel in managed_channels.iter_mut() {
                if channel.users.contains(&user) && Some(channel.id) != new_channel {
                    channel.users.remove(&user);
                } else if Some(channel.id) == new_channel {
                    channel.users.insert(user);
                }
            }

            managed_channels.retain(|channel| channel.users.len() > 0);
        }
    }
}