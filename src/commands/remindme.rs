use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use crate::models::reminder::Reminder;
use std::time::Duration;
use crate::REMINDERS;

pub struct RemindMe;

impl Command for RemindMe {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        let channel = msg.channel_id;
        let author = msg.author.id;
        let mut it = msg.content.split(" ").skip(1);
        if let Some(duration_arg) = it.next() {
            let content = it.collect::<Vec<&str>>().join(" ");
            let duration = match humantime::parse_duration(duration_arg) {
                Ok(dur) => dur,
                Err(_) => {
                    let dur = duration_arg.parse::<u64>()?;
                    Duration::from_secs(dur)
                }
            };
            REMINDERS.lock()?.push(
                Reminder::new(author.0, channel.0, content, duration));
        }

        Ok(())
    }
}
