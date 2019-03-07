use serenity::prelude::*;
use serenity::framework::standard::Command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError as Error;
use serenity::model::channel::Message;

#[derive(Deserialize)]
struct Cargo {
    package: Package
}

#[derive(Deserialize)]
struct Package {
    // name: String,
    version: String,
    // authors: Vec<String>,
    // edition: String
}

pub struct Version;

impl Command for Version {
    fn execute(&self, _: &mut Context, msg: &Message, _: Args) -> Result<(), Error> {
        let cargo: Cargo = toml::from_str(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")))?;
        msg.reply(&format!("Version {}", cargo.package.version))?;
        Ok(())
    }
}