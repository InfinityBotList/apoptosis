local bot = require("@omniplex-common/bot")
local discord = require("@omniplex-common/discord/apitypes")
local onEvent = require("../dispatch").onEvent
 
--- GuildAuditLogEntryCreate
---
--- Dispatched when a new audit log entry is created.
---
--- Provides said entry’s data and the id of the guild where it was created.
local function onGuildAuditLogEntryCreate(event: bot.DiscordEvent, callback: (entry: discord.AuditLogEntryObject) -> ())
    onEvent(event, "Discord.GUILD_AUDIT_LOG_ENTRY_CREATE", function(event)
        callback(event.data)
    end)
end

return onGuildAuditLogEntryCreate