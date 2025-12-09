local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildRoleDelete
---
--- Dispatched when a role is deleted in a guild.
local function onGuildRoleDelete(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildRoleDeletePayload) -> ())
    onEvent(event, "Discord.GUILD_ROLE_DELETE", function(event)
        callback(event.data)
    end)
end

return onGuildRoleDelete