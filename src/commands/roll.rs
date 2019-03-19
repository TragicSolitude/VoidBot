use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use rand::Rng;

pub struct Roll;

impl Command for Roll {
    fn execute(&self, _: &mut Context, msg: &Message, mut args: Args) -> Result<(), Error> {
        let mut rng = rand::thread_rng();
        let num_rolls = args.single::<usize>()?;
        let mut rolls = Vec::with_capacity(num_rolls);

        for _ in 0..num_rolls {
            rolls.push(rng.gen_range(1, 6).to_string());
        }

        let _ = msg.reply(&format!("rolled {}", rolls.join(", ")));

        Ok(())
    }
}
