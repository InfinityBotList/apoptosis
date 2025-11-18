--!nocheck

--- Permission type
export type Permission = {
    --- Namespace of the permission
    namespace: string,
    --- Permission of the permission
    perm: string,
    --- Whether the permission is negated
    negator: boolean,
}

local function permissionfromstring(perm_str: string): Permission
    error("implemented by rust")
end

--- PartialStaffPosition type
export type PartialStaffPosition = {
    id: string,
    index: number,
    perms: {Permission},
}

--- StaffPermissions type
export type StaffPermissions = {
    user_positions: {PartialStaffPosition},
    perm_overrides: {Permission},
    resolve: (self: StaffPermissions) -> ResolvedPermissions,
}

export type ResolvedPermissions = {
    __phantom: () -> (),
    perms: {Permission},
    hasperm: (self: ResolvedPermissions, perm: Permission) -> boolean,
}

local function haspermstr(perms: {string}, perm_str: string)
    error("implemented by rust")
end

-- Error types
export type CheckPatchChangesError = {
    type: "NoPermission",
    permission: Permission,
} | {
    type: "LacksNegatorForWildcard",
    wildcard: Permission,
    negator: Permission,
}

local function cpce_to_string(err: CheckPatchChangesError?): string
    if not err then
        return "An unknown check_patch_changes error occurred [is nil]"
    end

    if err.type == "NoPermission" then
        return "You do not have permission to change the permission: " .. tostring(err.permission)
    elseif err.type == "LacksNegatorForWildcard" then
        return "You do not have a negator for the wildcard permission: " .. tostring(err.wildcard) .. ", missing negator: " .. tostring(err.negator)
    else
        return "An unknown check_patch_changes error occurred"
    end
end

local function check_patch_changes(manager_perms: {Permission}, current_perms: {Permission}, new_perms: {Permission}): (boolean, CheckPatchChangesError?)
    error("implemented by rust")
end

local function check_patch_changes_str(manager_perms: {string}, current_perms: {string}, new_perms: {string})
    error("implemented by rust")
end

return {
    permissionfromstring = permissionfromstring,
    haspermstr = haspermstr,
    check_patch_changes = check_patch_changes,
    check_patch_changes_str = check_patch_changes_str,
    cpce_to_string = cpce_to_string,
}