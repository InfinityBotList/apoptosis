local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- MessagePollVoteRemove
---
--- Dispatched when a user removes their vote in a poll.
local function onMessagePollVoteRemove(event: bot.DiscordEvent, callback: (vote: discordGateway.MessagePollVoteRemove) -> ())
    onEvent(event, "Discord.MESSAGE_POLL_VOTE_REMOVE", function(event)
        callback(event.data)
    end)
end

return onMessagePollVoteRemove
