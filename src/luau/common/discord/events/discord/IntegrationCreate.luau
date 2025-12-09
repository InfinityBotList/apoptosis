local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- IntegrationCreate
---
--- Dispatched when an integration is created in a guild.
local function onIntegrationCreate(event: bot.DiscordEvent, callback: (integration: discordGateway.IntegrationCreatePayload) -> ())
    onEvent(event, "Discord.INTEGRATION_CREATE", function(event)
        callback(event.data)
    end)
end

return onIntegrationCreate
 