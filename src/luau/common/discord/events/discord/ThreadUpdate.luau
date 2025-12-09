local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- ThreadUpdate
---
--- Dispatched when a thread is updated.
local function onThreadUpdate(event: bot.DiscordEvent, callback: (event: discordGateway.UpdateThreadPayload) -> ())
    onEvent(event, "Discord.THREAD_UPDATE", function(event)
        callback(event.data)
    end)
end

return onThreadUpdate
