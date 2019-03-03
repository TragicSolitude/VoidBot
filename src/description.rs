use serenity::Error;
use serenity::model::gateway::Ready;

/// Edits or creates messages in text channels with "$VOIDBOT_DESCRIPTION" in
/// their topics with the text in DESCRIPTION.md.
/// 
/// # Arguments
/// * `data_about` - The ready state data about the bot, provided in the 'ready' 
/// function of the EventHandler trait
pub fn update_bot_description(data_about: Ready) -> Result<(), Error> {
    // TODO Figure out more generic error type that impl
    // convert::From<option::NoneError> so that we can clean up some of this
    // indentation
    for guild in data_about.guilds {
        for (_, guild_channel) in guild.id().channels()?.iter() {
            if let Some(topic) = &guild_channel.topic {
                if topic.contains("$VOIDBOT_DESCRIPTION") {
                    // TODO Split description if content.len() > 2000
                    let content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/DESCRIPTION.md"));
                    let mut messages = guild_channel.messages(|g| g.limit(100))?;
                    let mut edited = false;
                    for message in messages.iter_mut() {
                        if let Some(line) = message.content.lines().next() {
                            if line == "#!@" {
                                message.edit(|m| m.content(content))?;
                                edited = true;
                                break;
                            }
                        }
                    }

                    if !edited {
                        guild_channel.say(content)?;
                    }
                }
            }
        }
    }

    Ok(())
}