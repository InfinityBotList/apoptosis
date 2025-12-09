local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- MessageDeleteBulk
---
--- Dispatched when multiple messages are deleted at once.
local function onMessageDeleteBulk(event: bot.DiscordEvent, callback: (payload: discordGateway.MessageDeleteBulkPayload) -> ())
    onEvent(event, "Discord.MESSAGE_DELETE_BULK", function(event)
        callback(event.data)
    end)
end

return onMessageDeleteBulk
 