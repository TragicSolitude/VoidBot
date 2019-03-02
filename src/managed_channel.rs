use serenity::model::id::UserId;
use serenity::model::id::ChannelId;
use std::collections::HashSet;
use std::sync::Mutex;

lazy_static! {
    pub static ref MANAGED_CHANNELS: Mutex<Vec<ManagedChannel>> = Mutex::new(Vec::new());
}

pub struct ManagedChannel {
    pub id: ChannelId,
    pub users: HashSet<UserId>
}

impl Drop for ManagedChannel {
    fn drop(&mut self) {
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