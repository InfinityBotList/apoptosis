local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- ReactionRemoveEmoji
---
--- Dispatched when all reactions of a specific emoji are removed from a message.
local function onReactionRemoveEmoji(event: bot.DiscordEvent, callback: (payload: discordGateway.MessageReactionRemoveEmojiPayload) -> ())
    onEvent(event, "Discord.MESSAGE_REACTION_REMOVE_EMOJI", function(event)
        callback(event.data)
    end)
end

return onReactionRemoveEmoji
