local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
  
--- InviteDelete
---
--- Dispatched when an invite is deleted.
local function onInviteDelete(event: bot.DiscordEvent, callback: (data: discordGateway.InviteDeletePayload) -> ())
    onEvent(event, "Discord.INVITE_DELETE", function(event)
        callback(event.data)
    end)
end

return onInviteDelete
