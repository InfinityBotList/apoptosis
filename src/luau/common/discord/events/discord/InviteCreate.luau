local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- InviteCreate
---
--- Dispatched when an invite is created.
local function onInviteCreate(event: bot.DiscordEvent, callback: (data: discordGateway.InviteCreatePayload) -> ())
    onEvent(event, "Discord.INVITE_CREATE", function(event)
        callback(event.data)
    end)
end

return onInviteCreate
