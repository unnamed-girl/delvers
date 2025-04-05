# delvers
A blaseball-inspired dungeon crawling simulator.

Details and discussion can be found on [the crabitat discord](https://discord.gg/UWVxnPjs)

Crates:
- delver_sim: library crate contains all the actual sim logic.
- chronobase: library crate for an sql database that stores entities additively with a temporal_index
  - webserver: feature that adds a module for building and running a rocket webserver.
- game_runner: binary crates used for running the game/server.
- discord_bot: binary crate that runs a discord bot (need to add your bot's token)