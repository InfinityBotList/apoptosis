local discord = require "./apitypes"
local discordRest = require "./resttypes"
local lazy = require "../lazy"
local typesext = require "@omniplex-rust/typesext"

--- Options for getting audit logs in Discord
export type GetAuditLogOptions = {
    --- The action type to filter by
    action_type: discord.AuditLogEventType?,
    --- The user ID to filter by
    user_id: discord.Snowflake?,
    --- The audit log entry ID to filter
    before: discord.Snowflake?,
    --- The number of entries to return
    limit: number?
}

--- Options for getting an auto moderation rule in Discord
export type GetAutoModerationRuleOptions = {
    --- The rule ID
    rule_id: discord.Snowflake
}

--- Options for creating an auto moderation rule in Discord
export type CreateAutoModerationRuleOptions = {
    --- The reason for creating the rule
    reason: string,
    --- The data to create the rule with
    data: discordRest.CreateAutoModerationRuleRequest
}

--- Options for editing an auto moderation rule in Discord
export type EditAutoModerationRuleOptions = {
    --- The rule ID
    rule_id: discord.Snowflake,
    --- The reason for editing the rule
    reason: string,
    --- The data to edit the rule with
    data: discordRest.ModifyAutoModerationRuleRequest
}

--- Options for deleting an auto moderation rule in Discord
export type DeleteAutoModerationRuleOptions = {
    --- The rule ID
    rule_id: discord.Snowflake,
    --- The reason for deleting the rule
    reason: string
}

--- Options for editing a channel in Discord
export type EditChannelOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The reason for the edit
    reason: string,
    --- The data to edit the channel with
    data: discordRest.ModifyChannelRequest
}

--- Options for deleting a channel in Discord
export type DeleteChannelOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The reason for the deletion
    reason: string
}

--- Options for editting channel permissions
export type EditChannelPermissionsOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The target ID to edit permissions of
    target_id: discord.Snowflake,
    --- The allow permissions
    allow: typesext.MultiOption<string>,
    --- The deny permissions
    deny: typesext.MultiOption<string>,
    --- The type of the target
    kind: discord.OverwriteObjectType,
    --- The reason for the edit
    reason: string,
}

--- Options for editing a channel in Discord
export type CreateChannelOptions = {
    --- The reason for the create
    reason: string,
    --- The data to edit the channel with
    data: discordRest.CreateGuildChannelRequest
}

--- Options for adding a role to a member
export type AddGuildMemberRoleOptions = {
    --- The member ID
    user_id: discord.Snowflake,
    --- The role ID
    role_id: discord.Snowflake,
    --- The reason for adding the role
    reason: string
}

--- Options for removing a role from a member
export type RemoveGuildMemberRoleOptions = {
    --- The member ID
    user_id: discord.Snowflake,
    --- The role ID
    role_id: discord.Snowflake,
    --- The reason for adding the role
    reason: string
}

--- Options for removing a member from a guild
export type RemoveGuildMemberOptions = {
    --- The member ID
    user_id: discord.Snowflake,
    --- The reason for removing the member
    reason: string
}

--- Options for getting guild bans
---
--- Note: If both `before` and `after` are provided, `before` will take precedence.
export type GetGuildBansOptions = {
    --- The limit of bans to get (max 100)
    limit: number?,
    --- Before a certain user ID
    before: discord.Snowflake?,
    --- After a certain user ID
    after: discord.Snowflake?
}

--- Options for sending a message to a channel in Discord
export type CreateMessageOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The data to send the message with
    data: discordRest.CreateMessageRequest
}

export type ReactionType = {
    type: "Unicode",
    data: string
} | {
    type: "Custom",
    animated: boolean,
    id: discord.Snowflake,
    name: string?
}

--- Options for creating a reaction in Discord
export type CreateReactionOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The message ID
    message_id: discord.Snowflake,
    --- The reaction to add
    reaction: ReactionType
}

--- Options for deleting the reaction omniplex has made on Discord
export type DeleteOwnReactionOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The message ID
    message_id: discord.Snowflake,
    --- The reaction to add
    reaction: ReactionType
}

--- Options for deleting the reaction of a user on Discord
export type DeleteUserReactionOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The message ID
    message_id: discord.Snowflake,
    --- The user ID
    user_id: discord.Snowflake,
    --- The reaction to add
    reaction: ReactionType
}

--- Normal = normal reaction, burst is a super reaction
export type ReactionTypeEnum = "Normal" | "Burst"

--- Options for getting reactions on a message
export type GetReactionsOptions = {
    channel_id: discord.Snowflake,
    message_id: discord.Snowflake,
    reaction: ReactionType,
    type: ReactionTypeEnum?, -- Normal or burst/super reaction
    after: discord.Snowflake?, -- After which ID to use
    limit: number?,
}

--- Options for deleting all reactions on a message
export type DeleteAllReactionsForEmojiOptions = {
    channel_id: discord.Snowflake,
    message_id: discord.Snowflake,
    reaction: ReactionType,
}

export type EditMessageOptions = {
    channel_id: discord.Snowflake,
    message_id: discord.Snowflake,
    data: discordRest.EditMessageRequest    
}

--- Options for creating a command in Discord
export type CreateCommandOptions = {
    --- The data to create the command with
    data: discordRest.CreateGuildApplicationCommandRequest
}

--- Options for creating multiple command in Discord
export type CreateCommandsOptions = {
    --- The data to create the command with
    data: {discordRest.CreateGuildApplicationCommandRequest}
}

--- Options for creating an interaction response in Discord
export type CreateInteractionResponseOptions = {
    --- The interaction ID
    interaction_id: discord.Snowflake,
    --- The interaction token
    interaction_token: string,
    --- The data to create the interaction response with
    data: discordRest.CreateInteractionRequest
}

--- Options for editting an interaction response in Discord
export type EditInteractionResponseOptions = {
    --- The interaction token
    interaction_token: string,
    --- The data to edit the interaction response with
    data: discordRest.EditWebhookMessageRequest
}

--- Options for getting a followup message in Discord
export type GetFollowupMessageOptions = {
    --- The interaction token
    interaction_token: string,
    --- The message ID
    message_id: discord.Snowflake
}

--- Options for creating a followup message in Discord
export type CreateFollowupMessageOptions = {
    --- The interaction token
    interaction_token: string,
    --- The data to create the followup message with
    data: discordRest.CreateFollowupMessageRequest
}

--- Options for editting a followup message in Discord
export type EditFollowupMessageOptions = {
    --- The interaction token
    interaction_token: string,
    --- The message ID
    message_id: discord.Snowflake,
    --- The data to edit the followup message with
    data: discordRest.EditWebhookMessageRequest
}

--- Options for deleting a followup message in Discord
export type DeleteFollowupMessageOptions = {
    --- The interaction token
    interaction_token: string,
    --- The message ID
    message_id: discord.Snowflake
}

--- A message pagination object
export type MessagePagination = { type: "After" | "Around" | "Before", id: discord.Snowflake }

--- Options for getting messages in Discord
export type GetMessagesOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The target message
    target: MessagePagination?,
    --- The limit of messages to get
    limit: number?
}

--- Options for getting a message in Discord
export type GetMessageOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The message ID
    message_id: discord.Snowflake
}

--- Options for creating a guild ban in Discord
export type CreateGuildBanOptions = {
    --- The user ID to ban
    user_id: discord.Snowflake,
    --- The reason for the ban
    reason: string,
    --- The number of seconds to delete messages from
    delete_message_seconds: number?
}

--- Options for removing a guild ban in Discord
export type RemoveGuildBanOptions = {
    --- The user ID to unban
    user_id: discord.Snowflake,
    --- The reason for the unban
    reason: string
}

--- Options for getting guild members
export type GetGuildMembersOptions = {
    --- The limit of members to get
    limit: number?,
    --- The user ID to get members after
    after: discord.Snowflake?
}

--- Options for searching guild members
export type SearchGuildMembersOptions = {
    --- The query to search for
    query: string,
    --- The limit of members to get
    limit: number?,
}

--- Options for modifying a guild member
export type ModifyGuildMemberOptions = {
    --- The user ID to modify
    user_id: discord.Snowflake,
    --- The reason for the modification
    reason: string,
    --- The data to modify the member with
    data: discordRest.ModifyGuildMemberRequest
}

--- Options for modifying a guild
export type ModifyGuildOptions = {
    data: discordRest.ModifyGuildRequest,
    reason: string
}

--- Options for checking if a user has the needed Discord permissions to perform an action
export type omniplexCheckPermissionsOptions = {
    --- The user ID to check permissions for
    user_id: discord.Snowflake,
    --- The needed permissions
    needed_permissions: string
}

--- Options for checking if a user has the needed Discord permissions to perform an action 
--- and is above the target user in terms of hierarchy
export type omniplexCheckPermissionsAndHierarchyOptions = {
    --- The user ID to check permissions for
    user_id: discord.Snowflake,
    --- The target ID to check permissions for
    target_id: discord.Snowflake,
    --- The needed permissions
    needed_permissions: string
}

--- Extra/additional success response for checking if a user has the needed Discord permissions to perform an action
export type omniplexCheckPermissionsResponse = {
    --- The partial guild
    partial_guild: discord.Partial<discord.GuildObject>,
    --- The member
    member: discord.GuildMemberObject,
    --- The permissions
    permissions: string
}

export type omniplexCheckChannelPermissionsOptions = {
    --- The user ID to check permissions for
    user_id: discord.Snowflake,
    --- The channel ID to check permissions for
    channel_id: discord.Snowflake,
    --- The needed permissions
    needed_permissions: string
}

export type omniplexCheckChannelPermissionsResponse = {
    --- The partial guild
    partial_guild: discord.Partial<discord.GuildObject>,
    --- The channel
    channel: discord.ChannelObject,
    --- The member
    member: discord.GuildMemberObject,
    --- The permissions
    permissions: string
}

export type omniplexFusedMemberSingle = {
    --- The member
    member: discord.GuildMemberObject,
    --- The resolved permissions of the member in the guild
    resolved_perms: string
}

--- A fused member contains both a member, the guild and the resolved permissions of
--- the member in the guild. This is useful for operations that require both the member and the guild context, such as permission checks.
export type omniplexFusedMember = {
    --- The partial guild
    guild: discord.Partial<discord.GuildObject>,
    --- The member and resolved permissions
    members: {omniplexFusedMemberSingle}
}

--- Options for creating a guild role
export type CreateGuildRoleOptions = {
    --- The reason for the creation
    reason: string,
    --- The data to create the role with
    data: discordRest.CreateGuildRoleRequest
}

--- Options for modifying a guild role position
export type ModifyRolePositionOptions = {
    --- The data to modify the role position with
    data: {discordRest.ModifyGuildRolePositionsRequest},
    --- The reason for the modification
    reason: string
}

--- Options for modifying a guild role
export type EditGuildRoleOptions = {
    --- The reason for the creation
    reason: string,
    --- The data to create the role with
    data: discordRest.ModifyGuildRoleRequest,
    --- Role ID
    role_id: discord.Snowflake
}

--- Options for deleting a guild role
export type DeleteGuildRoleOptions = {
    --- The reason for the creation
    reason: string,
    --- Role ID
    role_id: discord.Snowflake
}

export type CreateChannelInviteOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The data to create the invite with
    data: discordRest.CreateChannelInviteRequest,
    --- The reason for the creation
    reason: string
}

export type DeleteChannelPermissionOptions = {
    --- The channel ID
    channel_id: discord.Snowflake,
    --- The overwrite ID
    overwrite_id: discord.Snowflake,
    --- The reason for the deletion
    reason: string
}

export type FollowAnnouncementChannelOptions = {
    --- The Channel ID
    channel_id: discord.Snowflake,
    --- Data
    data: discordRest.FollowAnnouncementChannelRequest,
    --- Reason
    reason: string,
}

--[[
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInviteOptions {
    pub code: String,
    pub with_counts: Option<bool>, // default to false
    pub with_expiration: Option<bool>, // default to false
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,    
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInviteOptions {
    pub code: String,
    pub reason: String,
}
]]
export type GetInviteOptions = {
    --- The invite code
    code: string,
    --- Whether to include counts
    with_counts: boolean?,
    --- Whether to include expiration
    with_expiration: boolean?,
    --- The guild scheduled event ID
    guild_scheduled_event_id: discord.Snowflake?
}

export type DeleteInviteOptions = {
    --- The invite code
    code: string,
    --- The reason for the deletion
    reason: string
}

--- Lazy<{discord.UserObject}>
type LazyUsersObject = lazy.Lazy<discord.UserObject>
-- Lazy<omniplexFusedMember>
type LazyomniplexFusedMember = lazy.Lazy<omniplexFusedMember>
--- Lazy<discord.AuditLogObject>
type LazyAuditLogObject = lazy.Lazy<discord.AuditLogObject>
--- Lazy<discord.ChannelObject>
type LazyChannelObject = lazy.Lazy<discord.ChannelObject>
--- Lazy<{discord.ChannelObject}>
type LazyChannelsObject = lazy.Lazy<{discord.ChannelObject}>
--- Lazy<discord.MessageObject> 
type LazyMessageObject = lazy.Lazy<discord.MessageObject>
--- Lazy<{discord.MessageObject}>
type LazyMessagesObject = lazy.Lazy<{discord.MessageObject}>
--- Lazy<discord.ApplicationCommandObject>
type LazyApplicationCommandObject = lazy.Lazy<discord.ApplicationCommandObject>
--- Lazy<{discord.ApplicationCommandObject}>
type LazyApplicationCommandsObject = lazy.Lazy<{discord.ApplicationCommandObject}>
--- Lazy<{discord.AutomoderationRuleObject}>
type LazyAutomoderationRuleObjectList = lazy.Lazy<{discord.AutomoderationRuleObject}>
--- Lazy<discord.AutomoderationRuleObject>
type LazyAutomoderationRuleObject = lazy.Lazy<discord.AutomoderationRuleObject>

--- Lazy<discord.GuildBanObject?>
type LazyBanOptionalObject = lazy.Lazy<discord.BanObject?>
--- Lazy<{discord.GuildBanObject}>
type LazyBanObjectList = lazy.Lazy<{discord.BanObject}>

--- Lazy<discord.GuildRoleObject>
type LazyRoleObject = lazy.Lazy<discord.GuildRoleObject>
--- Lazy<{[discord.Snowflake]: discord.GuildRoleObject}>
type LazyRolesObject = lazy.Lazy<{discord.GuildRoleObject}>
--- Lazy<discordRest.ListActiveGuildThreadsResponse>
type LazyActiveGuildThreadsResponse = lazy.Lazy<discordRest.ListActiveGuildThreadsResponse>
--- Lazy<discord.GuildObject>
type LazyGuildObject = lazy.Lazy<discord.GuildObject>
--- Lazy<discord.GuildPreviewObject>
type LazyGuildPreviewObject = lazy.Lazy<discord.GuildPreviewObject>
--- Lazy<discord.GuildMemberObject>
type LazyGuildMemberObject = lazy.Lazy<discord.GuildMemberObject>
--- Lazy<{discord.GuildMemberObject}>
type LazyGuildMembersObject = lazy.Lazy<{discord.GuildMemberObject}>
--- Lazy<{discord.InviteObject}>
type LazyInviteObjectList = lazy.Lazy<{discord.InviteObject}>
--- Lazy<discord.InviteObject>
type LazyInviteObject = lazy.Lazy<discord.InviteObject>
--- Lazy<discord.PresenceObject>
type LazyPresenceObject = lazy.Lazy<discord.PresenceObject>

--- DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.
export type DiscordExecutor = {
    guild_id: string,

    -- omniplex helpers

    --- @yields
    ---
    --- Checks an audit log reason for validity, errors out if invalid
    omni_check_reason: (self: DiscordExecutor, reason: string) -> nil,
    
    --- @yields
    ---
    --- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server
    omni_check_permissions: (self: DiscordExecutor, data: omniplexCheckPermissionsOptions) -> omniplexCheckPermissionsResponse,

    --- @yields
    ---
    --- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server and is above the target user with an ID of `data.target_id` in terms of hierarchy
    omni_check_permissions_and_hierarchy: (self: DiscordExecutor, data: omniplexCheckPermissionsAndHierarchyOptions) -> omniplexCheckPermissionsResponse,

    --- @yields
    ---
    --- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the channel `data.channel_id`
    omni_check_channel_permissions: (self: DiscordExecutor, data: omniplexCheckChannelPermissionsOptions) -> omniplexCheckChannelPermissionsResponse,

    --- @yields
    ---
    --- Fetches the member and resolved permissions of a user in the guild
    --- (also called a 'fused member' as it contains both the member and the resolved permissions)
    omni_get_fused_member: (self: DiscordExecutor, ids: {discord.Snowflake}) -> LazyomniplexFusedMember,

    -- Discord API

    -- Gateway

    --- @yields
    ---
    --- Returns the presence of a user by ID
    get_user_presence: (self: DiscordExecutor, user_id: discord.Snowflake) -> LazyPresenceObject,

    -- Audit Logs

    --- @yields
    ---
    --- Gets the audit logs
    get_audit_logs: (self: DiscordExecutor, data: GetAuditLogOptions) -> LazyAuditLogObject,

    -- Auto Moderation

    --- @yields
    --- 
    --- Lists the auto moderation rules available
    list_auto_moderation_rules: (self: DiscordExecutor) -> LazyAutomoderationRuleObjectList,

    --- @yields
    ---
    --- Gets an auto moderation rule by ID
    get_auto_moderation_rule: (self: DiscordExecutor, data: GetAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

    --- @yields
    ---
    --- Creates an auto moderation rule
    create_auto_moderation_rule: (self: DiscordExecutor, data: CreateAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

    --- @yields
    ---
    --- Edits an auto moderation rule
    edit_auto_moderation_rule: (self: DiscordExecutor, data: EditAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

    --- @yields
    ---
    --- Deletes an auto moderation rule
    delete_auto_moderation_rule: (self: DiscordExecutor, data: DeleteAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,
    
    -- Channel
    
    --- @yields
    ---
    --- Gets a channel
    get_channel: (self: DiscordExecutor, channel_id: string) -> LazyChannelObject, 

    --- @yields
    ---
    --- Edits a channel
    edit_channel: (self: DiscordExecutor, data: EditChannelOptions) -> LazyChannelObject,

    --- @yields
    ---
    --- Deletes a channel
    delete_channel: (self: DiscordExecutor, data: DeleteChannelOptions) -> LazyChannelObject,

    --- @yields
    ---
    --- Edits channel permissions for a target
    edit_channel_permissions: (self: DiscordExecutor, data: EditChannelPermissionsOptions) -> nil,

    --- @yields
    ---
    --- Gets all invites a channel has
    get_channel_invites: (self: DiscordExecutor) -> LazyInviteObjectList,

    --- @yields
    ---
    --- Creates a channel invite
    create_channel_invite: (self: DiscordExecutor, data: CreateChannelInviteOptions) -> LazyInviteObject,

    --- @yields
    ---
    --- Deletes a channel permission
    delete_channel_permission: (self: DiscordExecutor, data: DeleteChannelPermissionOptions) -> nil,

    --- @yields
    ---
    --- Follows a announcement channel
    follow_announcement_channel: (self: DiscordExecutor, data: FollowAnnouncementChannelOptions) -> LazyChannelObject,

    -- Guild
    
    --- @yields
    ---
    --- Gets the guild
    get_guild: (self: DiscordExecutor) -> LazyGuildObject,

    --- @yields
    ---
    --- Gets the guilds preview
    get_guild_preview: (self: DiscordExecutor) -> LazyGuildPreviewObject,

    --- @yields
    ---
    --- Edits the guild
    modify_guild: (self: DiscordExecutor, data: ModifyGuildOptions) -> LazyGuildObject,

    --- @yields
    ---
    --- Gets the guild channels
    get_guild_channels: (self: DiscordExecutor) -> LazyChannelsObject,

    --- @yields
    ---
    --- Creates a guild channel
    create_guild_channel: (self: DiscordExecutor, data: CreateChannelOptions) -> LazyChannelObject,

    --- @yields
    ---
    --- Modify guild channel positions. Only channels to be modified are required to be passed in `data`.
    modify_guild_channel_positions: (self: DiscordExecutor, data: {discordRest.ModifyGuildChannelPositionsRequest}) -> nil,

    --- @yields
    ---
    --- List active guild threads
    list_active_guild_threads: (self: DiscordExecutor) -> LazyActiveGuildThreadsResponse,

    --- @yields
    ---
    --- Gets a guild member by ID
    get_guild_member: (self: DiscordExecutor, user_id: string) -> LazyGuildMemberObject,

    --- @yields
    ---
    --- List all guild members
    list_guild_members: (self: DiscordExecutor, data: GetGuildMembersOptions) -> LazyGuildMembersObject,

    --- @yields
    ---
    --- Search guild members
    search_guild_members: (self: DiscordExecutor, data: SearchGuildMembersOptions) -> LazyGuildMembersObject,

    --- @yields
    ---
    --- Modify guild member (this includes timing out a member using `communication_disabled_until`)
    modify_guild_member: (self: DiscordExecutor, data: ModifyGuildMemberOptions) -> LazyGuildMemberObject,

    --- @yields
    ---
    --- Adds a role to a member
    add_guild_member_role: (self: DiscordExecutor, data: AddGuildMemberRoleOptions) -> nil,

    --- @yields
    ---
    --- Removes a role from a member
    remove_guild_member_role: (self: DiscordExecutor, data: RemoveGuildMemberRoleOptions) -> nil,

    --- @yields
    ---
    --- Removes a member from a guild
    remove_guild_member: (self: DiscordExecutor, data: RemoveGuildMemberOptions) -> nil,

    --- @yields
    ---
    --- Gets guild bans
    get_guild_bans: (self: DiscordExecutor, data: GetGuildBansOptions) -> LazyBanObjectList,

    --- @yields
    ---
    --- Gets a guild ban for a user or nil if it does not exist
    get_guild_ban: (self: DiscordExecutor, user_id: discord.Snowflake) -> LazyBanOptionalObject,

    --- @yields
    ---
    --- Creates a guild ban
    create_guild_ban: (self: DiscordExecutor, data: CreateGuildBanOptions) -> nil,

    --- @yields
    ---
    --- Removes a guild ban
    remove_guild_ban: (self: DiscordExecutor, data: RemoveGuildBanOptions) -> nil,

    --- @yields
    ---
    --- Returns the guild roles of a guild
    get_guild_roles: (self: DiscordExecutor) -> LazyRolesObject,

    --- @yields
    ---
    --- Returns a guild role by ID
    get_guild_role: (self: DiscordExecutor, role_id: discord.Snowflake) -> LazyRoleObject,

    --- @yields
    ---
    --- Creates a guild role
    create_guild_role: (self: DiscordExecutor, data: CreateGuildRoleOptions) -> LazyRoleObject,

    --- @yields
    ---
    --- Modify guild role positions
    modify_guild_role_positions: (self: DiscordExecutor, data: ModifyRolePositionOptions) -> LazyRolesObject,

    --- @yields
    ---
    --- Modifies a guild role
    modify_guild_role: (self: DiscordExecutor, data: EditGuildRoleOptions) -> LazyRoleObject,

    --- @yields
    ---
    --- Deletes a guild role
    delete_guild_role: (self: DiscordExecutor, data: DeleteGuildRoleOptions) -> (),

    -- Invites

    --- @yields
    ---
    --- Gets an invite by code
    get_invite: (self: DiscordExecutor, data: GetInviteOptions) -> LazyInviteObject,

    --- @yields
    ---
    --- Deletes an invite by code
    delete_invite: (self: DiscordExecutor, data: DeleteInviteOptions) -> LazyInviteObject,

    -- Messages

    --- @yields
    ---
    --- Gets messages from a channel
    get_channel_messages: (self: DiscordExecutor, data: GetMessagesOptions) -> LazyMessagesObject,

    --- @yields
    ---
    --- Gets a message
    get_channel_message: (self: DiscordExecutor, data: GetMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Creates a message
    create_message: (self: DiscordExecutor, data: CreateMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Crossposts a message to an announcement channel
    crosspost_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> LazyMessageObject,
    
    --- @yields
    ---
    --- Creates a reaction to a message
    create_reaction: (self: DiscordExecutor, data: CreateReactionOptions) -> nil,

    --- @yields
    --- Deletes the reaction omniplex has made on a message
    delete_own_reaction: (self: DiscordExecutor, data: DeleteOwnReactionOptions) -> nil,

    --- @yields
    --- Deletes a reaction another user has made on a message (see ``delete_own_reaction`` for omniplex's
    --- reactions)
    delete_user_reaction: (self: DiscordExecutor, data: DeleteUserReactionOptions) -> nil,

    --- @yields
    ---
    --- Gets all users who have reacted to awith the provided reaction based on provided criteria
    get_reactions: (self: DiscordExecutor, data: GetReactionsOptions) -> LazyUsersObject,

    --- @yields
    ---
    --- Deletes all reactions on a message
    delete_all_reactions: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> nil,

    --- @yields
    ---
    --- Deletes all reactions for a specific emoji on a message
    delete_all_reactions_for_emoji: (self: DiscordExecutor, data: DeleteAllReactionsForEmojiOptions) -> nil,

    --- @yields
    ---
    --- Edits a message
    edit_message: (self: DiscordExecutor, data: EditMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Deletes a message
    delete_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake, reason: string) -> nil,

    --- @yields
    ---
    --- Bulk deletes messages in a channel
    bulk_delete_messages: (self: DiscordExecutor, channel_id: discord.Snowflake, message_ids: {discord.Snowflake}, reason: string) -> nil,

    -- Interactions
    
    --- @yields
    ---
    --- Creates an interaction response
    create_interaction_response: (self: DiscordExecutor, data: CreateInteractionResponseOptions) -> nil,

    --- @yields
    ---
    --- Gets the original interaction response
    get_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> LazyMessageObject,

    --- @yields
    ---
    --- Edits the original interaction response
    edit_original_interaction_response: (self: DiscordExecutor, data: EditInteractionResponseOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Deletes the original interaction response
    delete_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> nil,

    --- @yields
    ---
    --- Gets a followup interaction response
    get_followup_message: (self: DiscordExecutor, data: GetFollowupMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Creates a followup interaction response
    create_followup_message: (self: DiscordExecutor, data: CreateFollowupMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Edits a followup interaction response
    edit_followup_message: (self: DiscordExecutor, data: EditFollowupMessageOptions) -> LazyMessageObject,

    --- @yields
    ---
    --- Deletes a followup interaction response
    delete_followup_message: (self: DiscordExecutor, data: DeleteFollowupMessageOptions) -> nil,

    -- Uncategorized (for now)
   
    --- @yields
    ---
    --- Gets all guild commands currently registered
    get_guild_commands: (self: DiscordExecutor) -> LazyApplicationCommandsObject,

    --- @yields
    ---
    --- Creates a guild command
    create_guild_command: (self: DiscordExecutor, data: CreateCommandOptions) -> LazyApplicationCommandObject,

    --- @yields
    ---
    --- Creates multiple guild commands
    create_guild_commands: (self: DiscordExecutor, data: CreateCommandsOptions) -> LazyApplicationCommandsObject,
}

export type Plugin = DiscordExecutor
 
return {}