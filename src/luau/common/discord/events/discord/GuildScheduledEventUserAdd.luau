local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildScheduledEventUserAdd
---
--- Dispatched when a user is added to a scheduled event in a guild.
local function onGuildScheduledEventUserAdd(event: bot.DiscordEvent, callback: (subscribed: discordGateway.GuildScheduledEventUserAddPayload) -> ())
    onEvent(event, "Discord.GUILD_SCHEDULED_EVENT_USER_ADD", function(event)
        callback(event.data)
    end)
end

return onGuildScheduledEventUserAdd
 