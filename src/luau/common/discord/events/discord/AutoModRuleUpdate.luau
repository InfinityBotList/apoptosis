local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- AutoModRuleUpdate
local function onAutoModRuleUpdate(event: bot.DiscordEvent, callback: (rule: discord.AutomoderationRuleObject) -> ())
    onEvent(event, "Discord.AUTO_MODERATION_RULE_UPDATE", function(event)
        callback(event.data)
    end)
end

return onAutoModRuleUpdate