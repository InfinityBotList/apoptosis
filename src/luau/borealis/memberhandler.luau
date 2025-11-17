--!strict
local bot = require"../common/bot"
local state = require"./state"
local function HandleMember(member: bot.FullGuildMemberObject, cacheServerInfo: bot.CacheServerInfo) 
    local borealis = state.GetState()
    if not member.member.user.bot then 
        local userStaffPerms = borealis.sharedlayer.getUserStaffPerms(member.member.user.id)
        local webmodrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.web_moderator_role)
        local staffrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.staff_role)

        if not webmodrole or not staffrole then
            error(`failed to find required roles for staff member {member.member.user.id} on server {member.guild_id}. Please verify that the web moderator role ({cacheServerInfo.web_moderator_role}) and staff role ({cacheServerInfo.staff_role}) exist in the server.`)
        end

        -- TODO: Finish this 
    end
end