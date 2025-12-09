local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- ChannelDelete
local function onChannelDelete(event: bot.DiscordEvent, callback: (channel: discord.ChannelObject) -> ())
    onEvent(event, "Discord.CHANNEL_DELETE", function(event)
        callback(event.data)
    end)
end

return onChannelDelete