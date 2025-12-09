local bot = require("@omniplex-common/bot")

local onEvent = require("../dispatch").onEvent
local discordGateway = require("@omniplex-common/discord/gatewayTypes")

--- WebhookUpdate
---
--- Dispatched when a channel’s webhook is updated.
local function onWebhookUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.WebhooksUpdatePayload) -> ())
    onEvent(event, "Discord.WEBHOOKS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onWebhookUpdate
