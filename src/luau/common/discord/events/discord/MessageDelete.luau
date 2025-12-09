local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- MessageDelete
---
--- Dispatched when a message is deleted.
local function onMessageDelete(event: bot.DiscordEvent, callback: (payload: discordGateway.MessageDeletePayload) -> ())
    onEvent(event, "Discord.MESSAGE_DELETE", function(event)
        callback(event.data)
    end)
end

return onMessageDelete
