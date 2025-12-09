local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ReactionRemove
---
--- Dispatched when a reaction is removed from a message.
local function onReactionRemove(event: bot.DiscordEvent, callback: (reaction: discordGateway.MessageReactionRemovePayload) -> ())
    onEvent(event, "Discord.MESSAGE_REACTION_REMOVE", function(event)
        callback(event.data)
    end)
end

return onReactionRemove
