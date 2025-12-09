local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- StageInstanceCreate
---
--- Dispatched when a stage instance is created.
local function onStageInstanceCreate(event: bot.DiscordEvent, callback: (instance: discord.StageInstanceObject) -> ())
    onEvent(event, "Discord.STAGE_INSTANCE_CREATE", function(event)
        callback(event.data)
    end)
end

return onStageInstanceCreate
