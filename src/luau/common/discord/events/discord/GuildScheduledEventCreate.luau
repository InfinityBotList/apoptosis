local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- GuildScheduledEventCreate
---
--- Dispatched when a scheduled event is created in a guild.
local function onGuildScheduledEventCreate(event: bot.DiscordEvent, callback: (event: discord.GuildScheduledEventObject) -> ())
    onEvent(event, "Discord.GUILD_SCHEDULED_EVENT_CREATE", function(event)
        callback(event.data)
    end)
end

return onGuildScheduledEventCreate