use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::time::SystemTime;
use serenity::model::id::ChannelId;
use serenity::model::id::UserId;
use serenity::framework::standard::CommandError as Error;
use serenity::prelude::*;
use crate::models::reminder::Reminder;

/// Tracks reminders set by users and periodically dumps the reminder queue to
/// "reminders.bin" for persistence across restarts
pub struct ReminderManager {
    reminders: Arc<Mutex<BinaryHeap<Reminder>>>
}

impl TypeMapKey for ReminderManager {
    type Value = Self;
}

impl ReminderManager {
    pub fn new() -> Self {
        let reminders =
            match File::open("persistence/reminders.bin") {
                Ok(mut f) => {
                    let mut bin = Vec::new();
                    let _ = f.read_to_end(&mut bin);
                    match bincode::deserialize::<BinaryHeap<Reminder>>(&bin) {
                        Ok(heap) => Arc::new(Mutex::new(heap)),
                        Err(_) => Arc::new(Mutex::new(BinaryHeap::new()))
                    }
                }
                Err(_) => Arc::new(Mutex::new(BinaryHeap::new()))
            };

        {
            let reminders = reminders.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(10));
                    if let Ok(lock) = reminders.lock() {
                        if let Ok(bin) = bincode::serialize(&*lock) {
                            if let Ok(mut f) = File::create("persistence/reminders.bin") {
                                let _ = f.write(&bin);
                            }
                        }
                    }
                }
            });
        }

        {
            let reminders = reminders.clone();
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    if let Ok(mut lock) = reminders.lock() {
                        while lock.len() > 0 && lock.peek().unwrap().expiration < SystemTime::now() {
                            let reminder = lock.pop().unwrap();
                            let _ = ChannelId::from(reminder.channel).say(
                                format!("{}: {}", UserId::from(reminder.author).mention(), reminder.message));
                        }
                    }
                }
            });
        }

        ReminderManager {
            reminders
        }
    }

    pub fn set_reminder(&mut self, reminder: Reminder) -> Result<(), Error> {
        self.reminders.lock()?.push(reminder);

        Ok(())
    }
}
