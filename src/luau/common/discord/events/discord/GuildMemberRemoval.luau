local bot = require("@omniplex-common/bot")

local discordGateway = require("@omniplex-common/discord/gatewayTypes")
local onEvent = require("../dispatch").onEvent

--- GuildMemberRemoval
---
--- Dispatched when a user’s membership ends by leaving, getting kicked, or being banned.
---
--- Provides the guild’s id and the user’s data.
local function onGuildMemberRemoval(event: bot.DiscordEvent, callback: (payload: discordGateway.GuildMemberRemovePayload) -> ())
    onEvent(event, "Discord.GUILD_MEMBER_REMOVE", function(event)
        callback(event.data)
    end)
end

return onGuildMemberRemoval