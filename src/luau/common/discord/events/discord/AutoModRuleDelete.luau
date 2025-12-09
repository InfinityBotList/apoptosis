local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- AutoModRuleDelete
local function onAutoModRuleDelete(event: bot.DiscordEvent, callback: (rule: discord.AutomoderationRuleObject) -> ())
    onEvent(event, "Discord.AUTO_MODERATION_RULE_DELETE", function(event)
        callback(event.data)
    end)
end

return onAutoModRuleDelete