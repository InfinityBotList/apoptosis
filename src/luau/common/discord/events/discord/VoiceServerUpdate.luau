local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
  
--- VoiceServerUpdate
---
--- Dispatched when a guild’s voice server was updated (or changed to another one).
local function onVoiceServerUpdate(event: bot.DiscordEvent, callback: (data: discordGateway.VoiceServerUpdatePayload) -> ())
    onEvent(event, "Discord.VOICE_SERVER_UPDATE", function(event)
        callback(event.data)
    end)
end

return onVoiceServerUpdate
