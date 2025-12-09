local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildStickersUpdate
---
--- Dispatched when the stickers in a guild are updated.
local function onGuildStickersUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildStickersUpdatePayload) -> ())
    onEvent(event, "Discord.GUILD_STICKERS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildStickersUpdate
 