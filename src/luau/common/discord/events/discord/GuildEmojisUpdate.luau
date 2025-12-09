local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildEmojisUpdate
---
--- Dispatched when the emojis in a guild are updated.
---
--- Provides the guild’s id and the new state of the emojis in the guild.
local function onGuildEmojisUpdate(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildEmojisUpdatePayload) -> ())
    onEvent(event, "Discord.GUILD_EMOJIS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onGuildEmojisUpdate