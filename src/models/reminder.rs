use std::cmp::Ordering;
use std::time::Instant;
use std::time::Duration;
use serenity::model::id::UserId;
use serenity::model::id::ChannelId;

pub struct Reminder {
    pub who: UserId,
    pub channel: ChannelId,
    pub message: String,
    pub expiration: Instant
}

impl Reminder {
    pub fn new(who: UserId, channel: ChannelId, message: String, offset: Duration) -> Self {
        let expiration = Instant::now() + offset;
        Reminder { who, channel, message, expiration }
    }
}

impl Eq for Reminder {}

impl PartialEq for Reminder {
    fn eq(&self, other: &Self) -> bool {
        self.expiration == other.expiration
    }
}

impl Ord for Reminder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for use in max-heap/priority-queue
        match self.expiration.cmp(&other.expiration) {
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater
        }
    }
}

impl PartialOrd for Reminder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}