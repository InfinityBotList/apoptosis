local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildIntegrationsUpdate
---
--- Dispatched when a guild’s integration is added, updated or removed.
local function onGuildIntegrationsUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildIntegrationsUpdatePayload) -> ())
    onEvent(event, "Discord.GUILD_INTEGRATIONS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildIntegrationsUpdate