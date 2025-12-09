local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- AutoModRuleCreate
local function onAutoModRuleCreate(event: bot.DiscordEvent, callback: (rule: discord.AutomoderationRuleObject) -> ())
    onEvent(event, "Discord.AUTO_MODERATION_RULE_CREATE", function(event)
        callback(event.data)
    end)
end

return onAutoModRuleCreate