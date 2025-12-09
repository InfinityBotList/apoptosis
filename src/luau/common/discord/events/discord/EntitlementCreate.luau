local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- EntitlementCreate
local function onEntitlementCreate(event: bot.DiscordEvent, callback: (entitlement: discord.EntitlementObject) -> ())
    onEvent(event, "Discord.ENTITLEMENT_CREATE", function(event)
        callback(event.data)
    end)
end

return onEntitlementCreate