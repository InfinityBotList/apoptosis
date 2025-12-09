local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- MessageUpdate
---
--- Provides the message update data, as well as the actual old and new message if data is available.
local function onMessageUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.MessageUpdatePayload) -> ())
    onEvent(event, "Discord.MESSAGE_UPDATE", function(event)
        callback(event.data)
    end)
end

return onMessageUpdate
