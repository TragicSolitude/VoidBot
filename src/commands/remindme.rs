use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use timer::Timer;
use std::sync::Arc;

lazy_static! {
    static ref REMINDERS: Arc<Mutex<Vec<timer::Guard>>> = Arc::new(Mutex::new(Vec::new()));
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
                println!("Schedule message");
                let guard = self.0.lock().schedule_with_delay(chrono::Duration::seconds(duration), move || {
                    println!("Send message");
                    let _ = channel.say(format!("{}: {}", author.mention(), content));
                    REMINDERS.lock().pop();
                });
                REMINDERS.lock().push(guard);
            }
        }

        Ok(())
    }
}