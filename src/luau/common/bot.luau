--!strict
local apitypes = require"./discord/apitypes"
local kittycat = require"@omniplex-rust/kittycat"

--- Full information about a guild member (user + member)
export type FullGuildMemberObject = {
    --- The guild id
    read guild_id: string,
    --- The member information
    read member: apitypes.GuildMemberObject,
    --- The user's presence information
    read presence: apitypes.PresenceObject?,
}

--- A shared bot interface
export type Bot = {
    --- Returns the user object given a user id
    read GetUser: (self: Bot, id: string) -> apitypes.UserObject,
    --- Returns the presence object given a user id and guild id
    read GetMember: (self: Bot, userid: string, guildid: string) -> apitypes.GuildMemberObject?,
    --- Returns all information about a userid/guildid (member+presence)
    read GetFullMember: (self: Bot, userid: string, guildid: string) -> FullGuildMemberObject?,
    --- Returns information about a role on a guild from cache
    read GetRole: (self: Bot, guildid: string, roleid: string) -> apitypes.GuildRoleObject?,
    --- Adds a role to a member
    read AddRole: (self: Bot, guildid: string, userid: string, roleid: string) -> (),
    --- Removes a role from a member
    read RemoveRole: (self: Bot, guildid: string, userid: string, roleid: string) -> (),
    --- Kicks a member from a guild
    read KickMember: (self: Bot, guildid: string, userid: string, reason: string) -> (),
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