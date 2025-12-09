local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent

--- CommandPermissionsUpdate
local function onCommandPermissionsUpdate(event: bot.DiscordEvent, callback: (permission: discord.GuildApplicationCommandPermissionsObject) -> ())
    onEvent(event, "Discord.APPLICATION_COMMAND_PERMISSIONS_UPDATE", function(event)
        callback(event.data)
    end)
end

return onCommandPermissionsUpdate