# ALE Discord Bot

Proof of Concept discord bot for [A Life Engineered](https://www.youtube.com/@ALifeEngineered)'s [discord](https://discord.gg/HFVMbQgRJJ)

# Current / Planned Features

- [ ] Collect the names of people who want to join the cohort.
  - [ ] Start automatically 1 week before the start of every month.
- [ ] Randomly pair them up
  - [ ] Avoiding people being re-paired with people they've been paired with before
- [ ] Post the pairs on the 1st of the month

# Configuration

For local runs please create and "fill" a .env file

- `TOKEN` [req] - provides the discord token.
- `COHORT_CHANNEL` [req] - specifies the channel ID to use for cohort notifications.
- `TEST_GUILD_ID` - provides the GUILD ID of the test server.
  If set it enables test mode.
  When in test mode slash commands are only registered on the test server not globally because they are available more quickly for testing.
- `STARTUP_MSG_CHANNEL` - If set bot will send a message in this channel when it starts up.
- `AUTH_ROLE_ID` - The role ID that can run privileged commands (Not Used right now).
