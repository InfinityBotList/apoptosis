local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- MessagePollVoteAdd
---
--- Dispatched when a user votes in a poll.
---
--- This will be dispatched multiple times if multiple answers are selected.
local function onMessagePollVoteAdd(event: bot.DiscordEvent, callback: (vote: discordGateway.MessagePollVoteAdd) -> ())
    onEvent(event, "Discord.MESSAGE_POLL_VOTE_ADD", function(event)
        callback(event.data)
    end)
end

return onMessagePollVoteAdd
