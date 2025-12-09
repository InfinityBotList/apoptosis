local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildRoleUpdate
---
--- Dispatched when a role is updated in a guild.
local function onGuildRoleUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildRoleUpdatePayload) -> ())
    onEvent(event, "Discord.GUILD_ROLE_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildRoleUpdate