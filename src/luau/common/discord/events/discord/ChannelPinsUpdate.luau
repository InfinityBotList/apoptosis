local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ChannelPinsUpdate
local function onChannelPinsUpdate(event: bot.DiscordEvent, callback: (pin: discordGateway.ChannelPinsUpdatePayload) -> ())
    onEvent(event, "Discord.CHANNEL_PINS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onChannelPinsUpdate