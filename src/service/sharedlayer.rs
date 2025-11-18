use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;
use serenity::all::UserId;

/// SharedLayer provides common methods across IBL's entire backend
/// to both
pub struct SharedLayer {
    pool: sqlx::PgPool,
}

impl SharedLayer {
    /// Creates a new SharedLayer
    ///
    /// Should be called once per service
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    /// Returns the user's staff permissions on Omni/IBL
    pub async fn get_user_staff_perms(&self, userid: UserId) -> kittycat::perms::StaffPermissions {
        todo!();
    }
}

impl LuaUserData for SharedLayer {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        /*
        --- Returns the user's staff permissions on Omni/IBL
        getUserStaffPerms: (userid: string) -> kittycat.Rust_StaffPermissions,
        --- Returns the bots state
        getBotState: (botid: string) -> string,
             */
    }
}
