use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use timer::Timer;
use std::sync::Arc;
use crate::models::reminder::Reminder;
use std::collections::BinaryHeap;
use chrono::Duration;

lazy_static! {
    /// Binary heap keeping all pending reminders in scope so that they don't
    /// get cancelled until they are manually dropped at the end of their
    /// closures
    static ref REMINDERS: Arc<Mutex<BinaryHeap<Reminder>>>
        = Arc::new(Mutex::new(BinaryHeap::new()));
}

pub struct RemindMe(pub Arc<Mutex<Timer>>);

impl Command for RemindMe {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        let channel = msg.channel_id;
        let author = msg.author.id;
        let mut it = msg.content.split(" ").skip(1);
        if let Some(duration_arg) = it.next() {
            if let Ok(duration) = duration_arg.parse::<i64>() {
                let content = it.collect::<Vec<&str>>().join("");
                let guard = self.0.lock().schedule_with_delay(
                    Duration::seconds(duration),
                    move || {
                        // Make sure REMINDERS is locked for the duration of the
                        // closure to reduce the possibility of the issue
                        // described below
                        let mut reminders_lock = REMINDERS.lock();

                        let _ = channel.say(
                            format!("{}: {}", author.mention(), content));

                        // Supposed to drop the current timer guard but seems to
                        // be pretty fragile. If messages are received at a rate
                        // higher than the precision of the reminder's
                        // expiration date then its possible for this to drop
                        // the wrong reminder and a timer guard to be stuck in
                        // limbo for eternity
                        reminders_lock.pop();
                    });
                REMINDERS.lock().push(Reminder::new(guard, duration));
            }
        }

        Ok(())
    }
}