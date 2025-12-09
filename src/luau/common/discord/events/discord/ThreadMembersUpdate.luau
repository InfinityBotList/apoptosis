local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- ThreadMembersUpdate
---
--- Dispatched when anyone is added to or removed from a thread.
local function onThreadMembersUpdate(event: bot.DiscordEvent, callback: (data: discordGateway.ThreadMembersUpdatePayload) -> ())
    onEvent(event, "Discord.THREAD_MEMBERS_UPDATE", function(event)
        callback(event.data)
    end)
end 

return onThreadMembersUpdate
