use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;
use regex::Regex;
use regex::Captures;

pub struct Eval;

impl Eval {
    fn exec(&self, _: &str) -> Result<f32, Error> {
        Ok(0f32)
    }
}

impl Command for Eval {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        let re = Regex::new(r"\(\((.*?)\)\)").unwrap();
        let content = msg.content
            .split(" ")
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" ");
        let ret = re.replace_all(&content, |caps: &Captures| {
            format!("{}", self.exec(&caps[1]).unwrap())
        });

        msg.reply(&*ret)?;

        Ok(())
    }
}
