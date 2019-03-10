use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::Arc;
use serenity::model::id::UserId;
use serenity::model::id::ChannelId;

lazy_static! {
    /// Contains a list of temporary channels currently being managed by the
    /// bot.
    pub static ref MANAGED_CHANNELS: Arc<Mutex<Vec<ManagedChannel>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Tracks channels that were created by the bot using the `!playing` command
/// along with the necessary stats required to efficiently check if the channel
/// needs to be pruned.
///
/// ManagedChannel implements the `Drop` trait so that the channel being managed
/// by a particular instance is automatically deleted when the instance goes out
/// of scope.
#[derive(Serialize, Deserialize)]
pub struct ManagedChannel {
    id: u64,
    users: HashSet<u64>
}

impl ManagedChannel {
    pub fn channel(&self) -> ChannelId {
        ChannelId::from(self.id)
    }

    pub fn has_user(&self, user: &UserId) -> bool {
        self.users.contains(user.as_u64())
    }

    pub fn insert_user(&mut self, user: UserId) -> bool {
        self.users.insert(u64::from(user))
    }

    pub fn remove_user(&mut self, user: &UserId) -> bool {
        self.users.remove(user.as_u64())
    }
}

impl Drop for ManagedChannel {
    fn drop(&mut self) {
        // Delete associated channel when instance goes out of scope
        let _ = self.channel().delete();
    }
}

impl ManagedChannel {
    pub fn new(id: ChannelId) -> Self {
        ManagedChannel {
            id: u64::from(id),
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
                if channel.has_user(&user) && Some(channel.channel()) != new_channel {
                    channel.remove_user(&user);
                } else if Some(channel.channel()) == new_channel {
                    channel.insert_user(user);
                }
            }

            managed_channels.retain(|channel| channel.users.len() > 0);
        }
    }
}
