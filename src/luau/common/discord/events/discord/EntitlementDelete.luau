local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- EntitlementDelete
---
--- Provides data about the subscription. Specifically, the ``entitlement.deleted`` field will be set.
local function onEntitlementDelete(event: bot.DiscordEvent, callback: (entitlement: discord.EntitlementObject) -> ())
    onEvent(event, "Discord.ENTITLEMENT_DELETE", function(event)
        callback(event.data)
    end)
end

return onEntitlementDelete