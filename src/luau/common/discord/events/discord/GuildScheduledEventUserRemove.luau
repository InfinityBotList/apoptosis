local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildScheduledEventUserRemove
---
--- Dispatched when a user is removed from a scheduled event in a guild.
local function onGuildScheduledEventUserRemove(event: bot.DiscordEvent, callback: (unsubscribed: discordGateway.GuildScheduledEventUserRemovePayload) -> ())
    onEvent(event, "Discord.GUILD_SCHEDULED_EVENT_USER_REMOVE", function(event)
        callback(event.data)
    end)
end

return onGuildScheduledEventUserRemove
 