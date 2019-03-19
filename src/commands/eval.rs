use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;

pub struct Eval;

impl Eval {
    fn exec(&self, exp: &str) -> Result<f64, Error> {
        meval::eval_str(exp).map_err(|e| Error(e.to_string()))
    }
}

impl Command for Eval {
    fn execute(&self, _: &mut Context, msg: &Message, args: Args) -> Result<(), Error> {
        let content = args.rest();
        let ret = match self.exec(content) {
            Ok(res) => format!("{}", res),
            Err(e) => format!("{:?}", e)
        };

        msg.reply(&format!("{} = {}", content, ret))?;

        Ok(())
    }
}
