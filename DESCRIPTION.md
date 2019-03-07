#!@
Does stuff

Has the following commands:
- `!version` Returns the current version of the bot
- `!ping` "pong"; this exists mostly just to test responsiveness of the bot
- `!playing [name]` Creates a temporary voice channel with the name `[name]` and
moves you into it. This channel is deleted when it has no users in it.
- `!remindme [time] [reminder]` Sends `[reminder]` in the channel the command
was run in mentioning the person who sent the command after `[time]` seconds
from the bot receiving the message.

**WARNING REGARDING `!remindme` COMMAND**
This command is currently quite fragile and has a number of known issues so use
it sparingly. Some of these issues include but are not limited to:
1. If 2 reminders are set within the same time interval there is a possibility
that one of the reminders will be forgotten.
2. The reminders are not backed with any kind of non-volatile storage and are
currently in-memory only. All pending reminders will be forgotten whenever the
bot restarts; because it is in active development it will restart often.
3. Reminders may occasionally be forgotten for no reason at all.
3. You can only input reminder times in seconds which is great for developing
with but not very user friendly.

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