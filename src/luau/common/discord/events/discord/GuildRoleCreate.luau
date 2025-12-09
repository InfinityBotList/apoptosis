local bot = require("@omniplex-common/bot")
local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildRoleCreate
---
--- Dispatched when a role is created in a guild.
local function onGuildRoleCreate(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildRoleCreatePayload) -> ())
    onEvent(event, "Discord.GUILD_ROLE_CREATE", function(event)
        callback(event.data)
    end)
end

return onGuildRoleCreate