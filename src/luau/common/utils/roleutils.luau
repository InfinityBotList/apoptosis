--!strict

local discord = require"../discord/discord"
local fullmember = require"./fullmember"

local function addRoleToFullMember(
    ctx: discord.DiscordExecutor,
    member: fullmember.FullGuildMemberObject, 
    roleid: string,
    reason: string
)  
    if table.find(member.member.roles, roleid) == nil then
        -- Role not already present, add it
        ctx:add_guild_member_role({
            user_id = member.member.user.id,
            role_id = roleid,
            reason = reason,
        })
    end
end

local function removeRoleFromFullMember(
    ctx: discord.DiscordExecutor,
    member: fullmember.FullGuildMemberObject, 
    roleid: string,
    reason: string
)  
    if table.find(member.member.roles, roleid) ~= nil then
        -- Role is present, remove it
        ctx:remove_guild_member_role({
            user_id = member.member.user.id,
            role_id = roleid,
            reason = reason,
        })
    end
end

return {
    addRoleToFullMember = addRoleToFullMember,
    removeRoleFromFullMember = removeRoleFromFullMember,
}