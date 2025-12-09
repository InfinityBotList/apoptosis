local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- IntegrationDelete
---
--- Dispatched when an integration is deleted in a guild.
local function onIntegrationDelete(event: bot.DiscordEvent, callback: (payload: discordGateway.IntegrationDeletePayload) -> ())
    onEvent(event, "Discord.INTEGRATION_DELETE", function(event)
        callback(event.data)
    end)
end

return onIntegrationDelete
 