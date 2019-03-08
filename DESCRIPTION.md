#!@
Does stuff

Has the following commands:
- `!version` Returns the current version of the bot
- `!ping` "pong"; this exists mostly just to test responsiveness of the bot
- `!playing [name]` Creates a temporary voice channel with the name `[name]` and
moves you into it. This channel is deleted when it has no users in it.
- `!remindme [time] [reminder]` Sends `[reminder]` in the channel the command
was run in mentioning the person who sent the command after `[time]` seconds
from the bot receiving the message. The reminder queue gets flushed to a file
every 10 seconds so in the event the bot restarts there is a possible 10 second
window of reminders that were forgotten but it's unlikely.

The bot also supports some additional passive functionality:
- If you want to have this description of the bot on your server, put
`$VOIDBOT_DESCRIPTION` into the topic of the text channel you would like to have
this description placed into. The bot will automatically update its description
on startup to reflect updates.

---

This bot is now open source. It uses the MIT license so its all available to
view, modify, copy, and distribute as long as the original copyright and license
text are included in any derivative works. It can be found here:

https://gitlab.com/TragicSolitude/voidbot

Bugs and feature requests can be logged in the issue tracker in this repository
or you can DM _OffensiveBias