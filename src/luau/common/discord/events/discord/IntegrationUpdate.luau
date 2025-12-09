local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- IntegrationUpdate
local function onIntegrationUpdate(event: bot.DiscordEvent, callback: (integration: discordGateway.IntegrationUpdatePayload) -> ())
    onEvent(event, "Discord.INTEGRATION_UPDATE", function(event)
        callback(event.data)
    end)
end

return onIntegrationUpdate
 