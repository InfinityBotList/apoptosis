local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ReactionRemoveAll
---
--- Dispatched when all reactions are removed from a message.
local function onReactionRemoveAll(event: bot.DiscordEvent, callback: (reaction: discordGateway.MessageReactionRemoveAllPayload) -> ())
    onEvent(event, "Discord.MESSAGE_REACTION_REMOVE_ALL", function(event)
        callback(event.data)
    end)
end

return onReactionRemoveAll
