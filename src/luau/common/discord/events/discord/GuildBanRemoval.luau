local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildBanRemoval
---
--- Dispatched when a user is unbanned from a guild.
---
--- Provides the user’s data and the id of the guild where the unban occurred.
local function onGuildBanRemoval(event: bot.DiscordEvent, callback: (user: discordGateway.GuildBanRemovePayload) -> ())
    onEvent(event, "Discord.GUILD_BAN_REMOVE", function(event)
        callback(event.data)
    end)
end 

return onGuildBanRemoval