local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- GuildScheduledEventDelete
---
--- Dispatched when a scheduled event is deleted in a guild.
local function onGuildScheduledEventDelete(event: bot.DiscordEvent, callback: (event: discord.GuildScheduledEventObject) -> ())
    onEvent(event, "Discord.GUILD_SCHEDULED_EVENT_DELETE", function(event)
        callback(event.data)
    end)
end

return onGuildScheduledEventDelete