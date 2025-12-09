local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ThreadCreate
---
--- Dispatched when a thread is created or the current user is added to a private thread.
local function onThreadCreate(event: bot.DiscordEvent, callback: (event: discordGateway.CreateThreadPayload) -> ())
    onEvent(event, "Discord.THREAD_CREATE", function(event)
        callback(event.data)
    end)
end

return onThreadCreate
