local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- ThreadListSync
---
--- Dispatched when the current user gains access to a thread.
local function onThreadListSync(event: bot.DiscordEvent, callback: (data: discordGateway.ThreadListSyncPayload) -> ())
    onEvent(event, "Discord.THREAD_LIST_SYNC", function(event)
        callback(event.data)
    end)
end

return onThreadListSync
