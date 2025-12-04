use super::cacheserver::CacheServerManager;
use super::kittycat as srv_kittycat;
use super::optional_value::OptionalValue;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;
use sqlx::Row;
use sqlx::types::Uuid;
use std::rc::Rc;

/// SharedLayer provides common methods across IBL's entire backend
/// to both
///
/// Ideally, every IBL apoptosis layer will have its own SharedLayer
#[derive(Clone)]
pub struct SharedLayer {
    pool: sqlx::PgPool,
    cache_server_manager: CacheServerManager,

    // Cache any computed fields here
    cache_server_manager_cache: Rc<OptionalValue<LuaAnyUserData>>,
}

impl SharedLayer {
    /// Creates a new SharedLayer
    ///
    /// Should be called once per layer
    #[allow(dead_code)]
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            cache_server_manager: CacheServerManager::new(pool.clone()),
            pool,
            cache_server_manager_cache: OptionalValue::new().into(),
        }
    }

    /// Returns the state of a bot by its user ID on Omni/IBL
    ///
    /// Returns None if the bot is not found
    pub async fn get_bot_state(&self, botid: String) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query("SELECT type FROM bots WHERE bot_id = $1")
            .bind(botid)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let state: String = row.try_get("type")?;
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Returns the user's staff permissions on Omni/IBL
    pub async fn get_user_staff_perms(
        &self,
        userid: String,
    ) -> Result<kittycat::perms::StaffPermissions, sqlx::Error> {
        let row =
            sqlx::query("SELECT positions, perm_overrides FROM staff_members WHERE user_id = $1")
                .bind(userid)
                .fetch_optional(&self.pool)
                .await?;

        let Some(row) = row else {
            return Ok(kittycat::perms::StaffPermissions {
                user_positions: vec![],
                perm_overrides: vec![],
            });
        };

        let positions: Vec<Uuid> = row.try_get("positions")?;
        let perm_overrides: Vec<String> = row.try_get("perm_overrides")?;

        let position_data =
            sqlx::query("SELECT id::text, index, perms FROM staff_positions WHERE id = ANY($1)")
                .bind(&positions)
                .fetch_all(&self.pool)
                .await?;

        let mut positions = Vec::with_capacity(position_data.len());

        for r in position_data {
            positions.push(kittycat::perms::PartialStaffPosition {
                id: r.try_get("id")?,
                index: r.try_get("index")?,
                perms: r
                    .try_get::<Vec<String>, _>("perms")?
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
            });
        }

        let sp = kittycat::perms::StaffPermissions {
            user_positions: positions,
            perm_overrides: perm_overrides.into_iter().map(|x| x.into()).collect(),
        };

        Ok(sp)
    }
}

impl LuaUserData for SharedLayer {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("CacheServerManager", |lua, this| {
            this.cache_server_manager_cache
                .get_failable(|| lua.create_any_userdata(this.cache_server_manager.shallow_clone()))
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_scheduler_async_method(
            "GetUserStaffPerms",
            |_lua, this, userid: String| async move {
                let perms = this
                    .get_user_staff_perms(userid)
                    .await
                    .map_err(LuaError::external)?;
                let lua_perms = srv_kittycat::StaffPermissions::from(perms);
                Ok(lua_perms)
            },
        );

        methods.add_scheduler_async_method("GetBotState", |_lua, this, botid: String| async move {
            let state = this
                .get_bot_state(botid)
                .await
                .map_err(LuaError::external)?;
            Ok(state)
        });
    }
}
