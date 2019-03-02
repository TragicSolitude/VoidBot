use serenity::prelude::*;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;

pub struct Ping;

impl Command for Ping {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), CommandError> {
        msg.reply("pong")?;

        Ok(())
    }
}