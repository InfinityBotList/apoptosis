local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- ThreadMemberUpdate
---
--- Dispatched when a thread member is updated.
local function onThreadMemberUpdate(event: bot.DiscordEvent, callback: (event: discordGateway.ThreadMemberUpdatePayload) -> ())
    onEvent(event, "Discord.THREAD_MEMBER_UPDATE", function(event)
        callback(event.data)
    end)
end

return onThreadMemberUpdate
