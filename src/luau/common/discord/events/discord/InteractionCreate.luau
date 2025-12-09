local bot = require("@omniplex-common/bot")

local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- InteractionCreate
---
--- Run an event on interaction create (e.g a slash command was used or a button was clicked)
local function onInteractionCreate(event: bot.DiscordEvent, callback: (interaction: discord.InteractionObject) -> ())
    onEvent(event, "Discord.INTERACTION_CREATE", function(event)
        callback(event.data)
    end)
end

return onInteractionCreate
