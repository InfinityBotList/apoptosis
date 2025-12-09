local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- TypingStart
---
--- Dispatched when a user starts typing in a channel.
local function onTypingStart(event: bot.DiscordEvent, callback: (data: discordGateway.TypingStartPayload) -> ())
    onEvent(event, "Discord.TYPING_START", function(event)
        callback(event.data)
    end)
end

return onTypingStart
