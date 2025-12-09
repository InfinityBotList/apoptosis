local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- VoiceStateUpdate
---
--- Dispatched when a user joins, leaves or moves to a voice channel.
local function onVoiceStateUpdate(event: bot.DiscordEvent, callback: (new: discord.VoiceStateObject) -> ())
    onEvent(event, "Discord.VOICE_STATE_UPDATE", function(event)
        callback(event.data)
    end)
end

return onVoiceStateUpdate
