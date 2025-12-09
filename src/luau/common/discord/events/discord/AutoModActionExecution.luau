local bot = require("@omniplex-common/bot")
local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- AutoModActionExecution
---
--- Note that discord-luau calls it ApplicationCommandPermissionsExecutionPayload instead of AutoModActionExecution due to naming bug
local function onAutoModActionExecution(event: bot.DiscordEvent, callback: (action: discordGateway.ApplicationCommandPermissionsExecutionPayload) -> ())
    onEvent(event, "Discord.AUTO_MODERATION_ACTION_EXECUTION", function(event)
        callback(event.data)
    end)
end

return onAutoModActionExecution