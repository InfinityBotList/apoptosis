--!strict

local bot = require"../bot"

local function addRoleToFullMember(
    member: bot.FullGuildMemberObject, 
    roleid: string,
    bot: bot.Bot
)  
    if table.find(member.member.roles, roleid) == nil then
        -- Role not already present, add it
        bot.addrole(member.guild_id, member.member.user.id, roleid)
    end
end

local function removeRoleFromFullMember(
    member: bot.FullGuildMemberObject, 
    roleid: string,
    bot: bot.Bot
)  
    if table.find(member.member.roles, roleid) ~= nil then
        -- Role is present, remove it
        bot.removerole(member.guild_id, member.member.user.id, roleid)
    end
end

return {
    addRoleToFullMember = addRoleToFullMember,
    removeRoleFromFullMember = removeRoleFromFullMember,
}