local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ReactionAdd
---
--- Dispatched when a reaction is added to a message.
local function onReactionAdd(event: bot.DiscordEvent, callback: (reaction: discordGateway.MessageReactionAddPayload) -> ())
    onEvent(event, "Discord.MESSAGE_REACTION_ADD", function(event)
        callback(event.data)
    end)
end

return onReactionAdd
 