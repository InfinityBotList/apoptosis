local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- StageInstanceDelete
---
--- Dispatched when a stage instance is deleted.
local function onStageInstanceDelete(event: bot.DiscordEvent, callback: (instance: discord.StageInstanceObject) -> ())
    onEvent(event, "Discord.STAGE_INSTANCE_DELETE", function(event)
        callback(event.data)
    end)
end

return onStageInstanceDelete
