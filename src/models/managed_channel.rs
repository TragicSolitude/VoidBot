use std::convert::From;
use serenity::model::id::ChannelId;

#[derive(Serialize, Deserialize)]
pub struct ManagedChannel(pub u64);

impl ManagedChannel {
    pub fn channel_id(&self) -> ChannelId {
        ChannelId::from(self.0)
    }
}

impl From<ChannelId> for ManagedChannel {
    fn from(id: ChannelId) -> Self {
        ManagedChannel(id.0)
    }
}

impl Drop for ManagedChannel {
    fn drop(&mut self) {
        let _ = ChannelId::from(self.0).delete();
    }
}

