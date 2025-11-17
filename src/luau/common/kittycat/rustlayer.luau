--!strict
local kittycat = require"./kittycat"

export type Rust_StaffPermissions = {
    user_positions: {kittycat.PartialStaffPosition},
    perm_overrides: {kittycat.Permission},
}

local function toStaffPermissions(rustPerms: Rust_StaffPermissions): kittycat.StaffPermissions
    return kittycat.StaffPermissions.new(rustPerms.user_positions, rustPerms.perm_overrides)
end

return {
    toStaffPermissions = toStaffPermissions,
}