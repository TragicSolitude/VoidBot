use std::cmp::Ordering;
use std::time::Duration;
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Reminder {
    pub author: u64,
    pub channel: u64,
    pub message: String,
    pub expiration: SystemTime
}

impl Reminder {
    pub fn new(author: u64, channel: u64, message: String, offset: Duration) -> Self {
        let expiration = SystemTime::now() + offset;
        Reminder { author, channel, message, expiration }
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