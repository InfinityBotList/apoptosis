--!strict
local apitypes = require"./apitypes"
local kittycat = require"./kittycat/rustlayer"

--- Full information about a guild member (user + member)
export type FullGuildMemberObject = {
    guild_id: string,
    member: apitypes.GuildMemberObject,
    presence: apitypes.PresenceObject?,
}

--- A shared bot interface
export type Bot = {
    --- Returns the user object given a user id
    getuser: (id: string) -> apitypes.UserObject,
    --- Returns the presence object given a user id and guild id
    getmember: (userid: string, guildid: string) -> apitypes.GuildMemberObject?,
    --- Returns all information about a userid/guildid (member+presence)
    getfullmember: (userid: string, guildid: string) -> FullGuildMemberObject?,
    --- Returns information about a role on a guild from cache
    getrole: (guildid: string, roleid: string) -> apitypes.GuildRoleObject?,
}

--- Information about a cache server
export type CacheServerInfo = {
    bots_role: string,
    system_bots_role: string,
    logs_channel: string,
    staff_role: string,
    web_moderator_role: string,
    name: string,
}

--- Common functions shared across the Omni/IBL backend
export type SharedLayer = {
    --- Returns the user's staff permissions on Omni/IBL
    getUserStaffPerms: (userid: string) -> {kittycat.Rust_StaffPermissions},
    --- Returns information about a cache server
    getCacheServerInfo: (guildid: string) -> CacheServerInfo,
}

return {}