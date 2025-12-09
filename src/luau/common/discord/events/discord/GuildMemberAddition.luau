local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildMemberAddition
---
--- Dispatched when a user joins a guild.
---
--- Provides the guild’s id and the user’s member data.
local function onGuildMemberAddition(event: bot.DiscordEvent, callback: (member: discordGateway.GuildMemberAddPayload) -> ())
    onEvent(event, "Discord.GUILD_MEMBER_ADD", function(event)
        callback(event.data)
    end)
end

return onGuildMemberAddition