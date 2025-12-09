local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- EntitlementUpdate
---
--- Provides data abut the updated subscription. If the entitlement is renewed, the ``entitlement.ends_at`` field will have changed.
local function onEntitlementUpdate(event: bot.DiscordEvent, callback: (entitlement: discord.EntitlementObject) -> ())
    onEvent(event, "Discord.ENTITLEMENT_UPDATE", function(event)
        callback(event.data)
    end)
end

return onEntitlementUpdate