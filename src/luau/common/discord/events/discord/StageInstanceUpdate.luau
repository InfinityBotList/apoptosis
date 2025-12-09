local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- StageInstanceUpdate
---
--- Dispatched when a stage instance is updated.
local function onStageInstanceUpdate(event: bot.DiscordEvent, callback: (new: discord.StageInstanceObject) -> ())
    onEvent(event, "Discord.STAGE_INSTANCE_UPDATE", function(event)
        callback(event.data)
    end)
end

return onStageInstanceUpdate
 