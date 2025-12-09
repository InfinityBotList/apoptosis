local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- ChannelCreate
local function onChannelCreate(event: bot.DiscordEvent, callback: (channel: discord.ChannelObject) -> ())
    onEvent(event, "Discord.CHANNEL_CREATE", function(event)
        callback(event.data)
    end)
end

return onChannelCreate