--!strict
local kittycat = require"./kittycat"

export type Rust_StaffPermissions = {
    user_positions: {kittycat.PartialStaffPosition},
    perm_overrides: {kittycat.Permission},
}

return {}