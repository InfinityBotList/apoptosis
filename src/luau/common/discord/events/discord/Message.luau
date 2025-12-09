local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- Message
---
--- Run an event on message. This is a helper function that extracts the useful data from the event and calls the callback.
local function onMessage(event: bot.DiscordEvent, callback: (msg: discordGateway.MessageCreatePayload) -> ())
    onEvent(event, "Discord.MESSAGE", function(event)
        callback(event.data)
    end)
end

return onMessage 
