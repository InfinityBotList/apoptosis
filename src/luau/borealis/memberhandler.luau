--!strict
local bot = require"../common/bot"
local state = require"./state"
local rustlayer = require"../common/kittycat/rustlayer"
local kittycat = require"../common/kittycat/kittycat"
local roleutils = require"../common/utils/roleutils"
local miscutils = require"../common/utils/misc"

local BOREALIS_CAN_HAVE_STAFF_ROLE = kittycat.Permission.from_string("borealis.can_have_staff_role")
local BOT_REVIEWER_MARKER = kittycat.Permission.from_string("bot_reviewer.marker")

local function handleMemberNotBot(member: bot.FullGuildMemberObject, cacheServerInfo: state.CacheServerInfo)
    local borealis = state.GetState()

    local webmodrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.web_moderator_role)
    local staffrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.staff_role)

    if not webmodrole or not staffrole then
        error(`failed to find required roles for staff member {member.member.user.id} on server {member.guild_id}. Please verify that the web moderator role ({cacheServerInfo.web_moderator_role}) and staff role ({cacheServerInfo.staff_role}) exist in the server.`)
    end

    local userStaffPerms = borealis.sharedlayer.getUserStaffPerms(member.member.user.id)
    local resolved = rustlayer.toStaffPermissions(userStaffPerms):resolve()

    if kittycat.has_perm(resolved, BOREALIS_CAN_HAVE_STAFF_ROLE) then 
        print(`Adding staff role to user {member.member.user.id} on server {member.guild_id}`)
        roleutils.addRoleToFullMember(member, cacheServerInfo.staff_role, borealis.bot)
    else 
        print(`Removing staff role from user {member.member.user.id} on server {member.guild_id}`)
        roleutils.removeRoleFromFullMember(member, cacheServerInfo.staff_role, borealis.bot)
    end

    if kittycat.has_perm(resolved, BOT_REVIEWER_MARKER) then 
        print(`Adding web moderator role to user {member.member.user.id} on server {member.guild_id}`)
        roleutils.addRoleToFullMember(member, cacheServerInfo.web_moderator_role, borealis.bot)
    else 
        print(`Removing web moderator role from user {member.member.user.id} on server {member.guild_id}`)
        roleutils.removeRoleFromFullMember(member, cacheServerInfo.web_moderator_role, borealis.bot)
    end
end

local function handleMemberBot(member: bot.FullGuildMemberObject, cacheServerInfo: state.CacheServerInfo)
    local borealis = state.GetState()

    -- Check if bot is a needed bot
    for _, bot in borealis.config.needed_bots do 
        if bot.id == member.member.user.id then 
            -- Give Needed Bots and Bots role
            local botsrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.bots_role)
            if not botsrole then
                error(`failed to find bots role for needed bot {member.member.user.id} on server {member.guild_id}. Please verify that the bots role ({cacheServerInfo.bots_role}) exist in the server.`)
            end
            local neededbotsrole = borealis.bot.getrole(member.guild_id, cacheServerInfo.system_bots_role)
            if not neededbotsrole then
                error(`failed to find system bots role for needed bot {member.member.user.id} on server {member.guild_id}. Please verify that the system bots role ({cacheServerInfo.system_bots_role}) exist in the server.`)
            end

            print(`Adding needed bot roles to bot {member.member.user.id} on server {member.guild_id}`)
            roleutils.addRoleToFullMember(member, cacheServerInfo.bots_role, borealis.bot)
            roleutils.addRoleToFullMember(member, cacheServerInfo.system_bots_role, borealis.bot)
            return
        end
    end

    local botCacheServerId = borealis.cacheserver.lookupBot(member.member.user.id)
    if botCacheServerId ~= member.guild_id then 
        borealis.bot.kickmember(member.guild_id, member.member.user.id, "Bot is not in its designated cache server")
        print(`Kicked bot {member.member.user.id} from server {member.guild_id} as it is not in its designated cache server {botCacheServerId}`)
        return
    end

    assert(botCacheServerId) -- At this point it must be defined

    local botState = borealis.sharedlayer.getBotState(member.member.user.id)
    if not miscutils.isApprovedState(botState) then 
        borealis.cacheserver.removeBot(member.member.user.id)
        borealis.bot.kickmember(member.guild_id, member.member.user.id, `Bot is in unapproved state: {botState}`)
        print(`Kicked bot {member.member.user.id} from server {member.guild_id} as it is in unapproved state: {botState}`)
        return
    end

    roleutils.addRoleToFullMember(member, cacheServerInfo.bots_role, borealis.bot)
end

local function HandleMember(member: bot.FullGuildMemberObject, cacheServerInfo: state.CacheServerInfo) 
    if member.member.user.bot then 
        handleMemberBot(member, cacheServerInfo)
    else
        handleMemberNotBot(member, cacheServerInfo)
    end
end

return {
    HandleMember = HandleMember,
}