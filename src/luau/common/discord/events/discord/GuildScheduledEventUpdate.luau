local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildScheduledEventUpdate
---
--- Dispatched when a scheduled event is updated in a guild.
local function onGuildScheduledEventUpdate(event: bot.DiscordEvent, callback: (event: discord.GuildScheduledEventObject) -> ())
    onEvent(event, "Discord.GUILD_SCHEDULED_EVENT_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildScheduledEventUpdate