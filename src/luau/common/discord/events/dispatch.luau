local bot = require("@omniplex-common/bot")


--- Function that checks if the event matches the name.
---
--- An event matches name when its ``base_name`` or ``name`` is equal to the ``name`` argument or when ``{base_name}.{name}`` is equal to the ``name`` argument.
--- 
--- For example, using either ``Message`` or ``Discord.Message`` as name would match a Discord Message event.
---
--- **Note that this function is case-insensitive and internally converts all strings to lowercase.**
--- @param pat string The name to match against.
--- @param event bot.DiscordEvent The event to check.
--- @return boolean True if the event matches the name, false otherwise.
local function doesEventMatch(pat: string, event: bot.DiscordEvent)
    local baseName = string.lower(event.base_name)
    local name = string.lower(event.name)
    local pattern = string.lower(pat)
    
    return baseName == pattern or name == pattern or (baseName .. "." .. name) == pattern
end

--- Simple helper method that calls ``callback`` when the event matches the name. See ``doesEventMatch`` for more information
--- on how the name is matched.
---
--- @param event Primitives.Event The event to listen for.
--- @param pats string | {string} The name or list of names to match against.
--- @param callback function The callback to run when the event matches the name. The callback will receive the event as an argument.
--- @return nil
local function onEvent(event: bot.DiscordEvent, pats: string | {string}, callback: (bot.DiscordEvent) -> ())
    if type(pats) == "string" then
        if doesEventMatch(pats, event) then
            return callback(event)
        end
    else
        for _, pat in pats do
            if doesEventMatch(pat, event) then
                return callback(event)
            end
        end
    end
    return
end

--- Simple helper method that calls ``callback`` when the event matches the name. See ``doesEventMatch`` for more information
--- on how the name is matched.
local function onEventGeneric<R>(event: bot.DiscordEvent, pats: string | {string}, callback: (bot.DiscordEvent) -> R): R? 
    if type(pats) == "string" then
        if doesEventMatch(pats, event) then
            return callback(event)
        end
    else
        for _, pat in pats do
            if doesEventMatch(pat, event) then
                return callback(event)
            end
        end
    end

    return nil
end

assert(doesEventMatch("Discord.INTERACTION_CREATE", {base_name = "Discord", name = "INTERACTION_CREATE"}))

return {
    doesEventMatch = doesEventMatch,
    onEvent = onEvent,
    onEventGeneric = onEventGeneric
}