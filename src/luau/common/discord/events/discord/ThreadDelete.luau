local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- ThreadDelete
---
--- Dispatched when a thread is deleted.
---
--- Provides the partial data about the deleted thread and, if it was present in the cache before its deletion, its full data.
local function onThreadDelete(event: bot.DiscordEvent, callback: (event: discordGateway.DeleteThreadPayload) -> ())
    onEvent(event, "Discord.THREAD_DELETE", function(event)
        callback(event.data)
    end)
end

return onThreadDelete
