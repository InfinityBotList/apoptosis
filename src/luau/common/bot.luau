--!strict
local kittycat = require"@omniplex-rust/kittycat"
local discord = require"./discord/discord"

--- A shared bot interface
export type Bot = {
    --- Returns a discord provider given guild id to use for operations
    read HTTPForGuild: (self: Bot, guildid: string) -> discord.DiscordExecutor,
}


--- Information about a cache server
export type CacheServerInfo = {
    bots_role: string,
    system_bots_role: string,
    logs_channel: string,
    staff_role: string,
    web_moderator_role: string,
    name: string,
    invite_code: string,
}

export type CacheServerManager = {
    --- Returns information about a cache server
    read Get: (self: CacheServerManager, guildid: string) -> CacheServerInfo?,
    --- Returns the cache server id for a bot given its bot id
    read LookupBot: (self: CacheServerManager, botid: string) -> string?,
    --- Removes a bot from its cache server
    read RemoveBot: (self: CacheServerManager, botid: string) -> (),
    --- Deletes a cache server
    read Delete: (self: CacheServerManager, guildid: string) -> (),
}

--- Common functions shared across the Omni/IBL backend
export type SharedLayer = {
    --- Returns the user's staff permissions on Omni/IBL
    read GetUserStaffPerms: (self: SharedLayer, userid: string) -> kittycat.StaffPermissions,
    --- Returns the bots state
    read GetBotState: (self: SharedLayer, botid: string) -> string,
    --- Returns information about a cache server
    read CacheServer: CacheServerManager
}

--- The context passed as the argument to all omniplex layer invokations
export type Context<BaseLayer, LayerEvent> = {
    read layer: BaseLayer,
    read event: LayerEvent,
}

--- A LayerThread is a special userdata representing a thread through which events can be sent to other layers
export type LayerThread<LayerEvent> = {
    --- Dispatches an event to the layer thread
    read Dispatch: (self: LayerThread<LayerEvent>, event: LayerEvent) -> (),
}

return {}