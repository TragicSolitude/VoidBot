#!@
Does stuff

Has the following commands:
- `!version` Returns the current version of the bot
- `!ping` "pong"; this exists mostly just to test responsiveness of the bot
- `!playing [name]` Creates a temporary voice channel with the name `[name]` and
moves you into it. This channel is deleted when it has no users in it.

The bot also supports some additional passive functionality:
- If you want to have this description of the bot on your server, put
`$VOIDBOT_DESCRIPTION` into the topic of the text channel you would like to have
this description placed into. The bot will automatically update its description
on startup to reflect updates.