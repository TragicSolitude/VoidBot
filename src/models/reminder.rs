use std::cmp::Ordering;
use timer::Guard;
use chrono::Utc;

pub struct Reminder {
    pub guard: Guard,
    pub expiration: i64
}

impl Reminder {
    pub fn new(guard: Guard, duration: i64) -> Self {
        let now = Utc::now().timestamp_millis();
        let expiration = now + (duration * 1000);
        Reminder { guard, expiration }
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