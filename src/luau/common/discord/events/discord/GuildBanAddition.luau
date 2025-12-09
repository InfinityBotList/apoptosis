local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildBanAddition
---
--- Dispatched when a user is banned from a guild.
---
--- Provides the user’s data and the id of the guild where the ban occurred.
local function onGuildBanAddition(event: bot.DiscordEvent, callback: (data: discordGateway.GuildBanAddPayload) -> ())
    onEvent(event, "Discord.GUILD_BAN_ADD", function(event)
        callback(event.data)
    end)
end

return onGuildBanAddition