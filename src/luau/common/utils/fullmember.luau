--!strict

local apitypes = require"../discord/apitypes"
local discord = require"../discord/discord"

--- Full information about a guild member (user + member)
export type FullGuildMemberObject = {
    --- The guild id
    read guild_id: string,
    --- The member information
    read member: apitypes.GuildMemberObject,
    --- The user's presence information
    read presence: apitypes.PresenceObject?,
}


local function GetFullMember(ctx: discord.DiscordExecutor, userid: string): FullGuildMemberObject?
    local member = ctx:get_guild_member(userid)
    local presence = ctx:get_user_presence(userid)
    return {
        guild_id = ctx.guild_id,
        member = member.data,
        presence = presence.data,
    }
end

return {
    GetFullMember = GetFullMember,
}