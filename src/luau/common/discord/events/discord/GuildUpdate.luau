local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildUpdate
---
--- Dispatched when the guild is updated.
---
--- Note that the new data may be partial
local function onGuildUpdate(event: bot.DiscordEvent, callback: (new: discord.GuildObject) -> ())
    onEvent(event, "Discord.GUILD_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildUpdate
 