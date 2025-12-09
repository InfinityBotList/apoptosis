local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- ChannelUpdate
local function onChannelUpdate(event: bot.DiscordEvent, callback: (channel: discord.ChannelObject) -> ())
    onEvent(event, "Discord.CHANNEL_UPDATE", function(event)
        callback(event.data)
    end)
end

return onChannelUpdate