#!@
Does stuff

Has the following commands:
- `!version` Returns the current version of the bot
- `!ping` "pong"; this exists mostly just to test responsiveness of the bot
- `!playing [name]` Creates a temporary voice channel with the name `[name]` and
moves you into it. This channel is deleted when it has no users in it.
- `!allplaying [name]` Does the same thing as the `!playing` command but brings
everyone else in your channel with you.
- `!remindme [time] [reminder]` Sends `[reminder]` in the channel the command
was run in mentioning the person who sent the command after `[time]` from the
bot receiving the message. `[time]` can be specified as either a plain number in
which it is interpreted as seconds or it can have some kind of suffix to specify
the interval. For example; 32 minutes can be represented as `32m` or `32minutes`
or `1920seconds`. This works for "seconds," "minutes," "hours," "days," and even
"years."
- `!eval [expression]` Evaluates basic math and responds to the caller with the
result For example: `!eval 1 + 1` will be responded to with `@User: 2`.
- `!roll [n]` "Roll the dice." Returns `[n]` random numbers between 1 and 6
inclusive.

All stateful commands (e.g. `!playing` and `!remindme`) have relevant data
structures periodically dumped to the filesystem so the bot should for the most
part survive restarts gracefully.

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

