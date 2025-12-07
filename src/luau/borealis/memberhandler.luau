--!strict
local bot = require"../common/bot"
local discord = require"../common/discord/discord"
local fullmember = require"../common/utils/fullmember"
local state = require"./state"
local kittycat = require"@omniplex-rust/kittycat"
local roleutils = require"../common/utils/roleutils"
local miscutils = require"../common/utils/misc"

local BOREALIS_CAN_HAVE_STAFF_ROLE = kittycat.permissionfromstring("borealis.can_have_staff_role")
local BOT_REVIEWER_MARKER = kittycat.permissionfromstring("bot_reviewer.marker")

local function handleMemberNotBot(borealis: state.Borealis, ctx: discord.DiscordExecutor, member: fullmember.FullGuildMemberObject, cacheServerInfo: bot.CacheServerInfo)
    local webmodrole = ctx:get_guild_role(cacheServerInfo.web_moderator_role)
    local staffrole = ctx:get_guild_role(cacheServerInfo.staff_role)

    if not webmodrole or not staffrole then
        error(`failed to find required roles for staff member {member.member.user.id} on server {member.guild_id}. Please verify that the web moderator role ({cacheServerInfo.web_moderator_role}) and staff role ({cacheServerInfo.staff_role}) exist in the server.`)
    end

    local userStaffPerms = borealis.sharedlayer:GetUserStaffPerms(member.member.user.id)
    local resolved = userStaffPerms:resolve()

    if resolved:hasperm(BOREALIS_CAN_HAVE_STAFF_ROLE) then 
        print(`Adding staff role to user {member.member.user.id} on server {member.guild_id}`)
        roleutils.addRoleToFullMember(ctx, member, cacheServerInfo.staff_role, "Granted borealis.can_have_staff_role")
    else 
        print(`Removing staff role from user {member.member.user.id} on server {member.guild_id}`)
        roleutils.removeRoleFromFullMember(ctx, member, cacheServerInfo.staff_role, "Revoked borealis.can_have_staff_role")
    end

    if resolved:hasperm(BOT_REVIEWER_MARKER) then 
        print(`Adding web moderator role to user {member.member.user.id} on server {member.guild_id}`)
        roleutils.addRoleToFullMember(ctx, member, cacheServerInfo.web_moderator_role, "Granted bot_reviewer.marker")
    else 
        print(`Removing web moderator role from user {member.member.user.id} on server {member.guild_id}`)
        roleutils.removeRoleFromFullMember(ctx, member, cacheServerInfo.web_moderator_role, "Revoked bot_reviewer.marker")
    end
end

local function handleMemberBot(borealis: state.Borealis, ctx: discord.DiscordExecutor, member: fullmember.FullGuildMemberObject, cacheServerInfo: bot.CacheServerInfo)
    -- Check if bot is a needed bot
    for _, bot in borealis.config.needed_bots do 
        if bot.id == member.member.user.id then 
            -- Give Needed Bots and Bots role
            local botsrole = ctx:get_guild_role(cacheServerInfo.bots_role)
            if not botsrole then
                error(`failed to find bots role for needed bot {member.member.user.id} on server {member.guild_id}. Please verify that the bots role ({cacheServerInfo.bots_role}) exist in the server.`)
            end
            local neededbotsrole = ctx:get_guild_role(cacheServerInfo.system_bots_role)
            if not neededbotsrole then
                error(`failed to find system bots role for needed bot {member.member.user.id} on server {member.guild_id}. Please verify that the system bots role ({cacheServerInfo.system_bots_role}) exist in the server.`)
            end

            print(`Adding needed bot roles to bot {member.member.user.id} on server {member.guild_id}`)
            roleutils.addRoleToFullMember(ctx, member, cacheServerInfo.bots_role, "Needed Bot")
            roleutils.addRoleToFullMember(ctx, member, cacheServerInfo.system_bots_role, "System Needed Bot")
            return
        end
    end

    local botCacheServerId = borealis.sharedlayer.CacheServer:LookupBot(member.member.user.id)
    if botCacheServerId ~= member.guild_id then 
        ctx:remove_guild_member({
            user_id = member.member.user.id, 
            reason = "Bot is not in its designated cache server"
        })
        print(`Kicked bot {member.member.user.id} from server {member.guild_id} as it is not in its designated cache server {botCacheServerId}`)
        return
    end

    assert(botCacheServerId) -- At this point it must be defined

    local botState = borealis.sharedlayer:GetBotState(member.member.user.id)
    if not miscutils.isApprovedState(botState) then 
        borealis.sharedlayer.CacheServer:RemoveBot(member.member.user.id)
        ctx:remove_guild_member({
            user_id = member.member.user.id, 
            reason = `Bot is in unapproved state: {botState}`
        })
        print(`Kicked bot {member.member.user.id} from server {member.guild_id} as it is in unapproved state: {botState}`)
        return
    end

    roleutils.addRoleToFullMember(ctx, member, cacheServerInfo.bots_role, "Bot is in approved state")
end

local function HandleMember(borealis: state.Borealis, ctx: discord.DiscordExecutor, member: fullmember.FullGuildMemberObject, cacheServerInfo: bot.CacheServerInfo) 
    if member.member.user.bot then 
        handleMemberBot(borealis, ctx, member, cacheServerInfo)
    else
        handleMemberNotBot(borealis, ctx, member, cacheServerInfo)
    end
end

return {
    HandleMember = HandleMember,
}