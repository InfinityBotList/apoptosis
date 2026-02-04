use crate::Db;
use crate::entity::EntityType;
use crate::entity::manager::EntityManager;
use crate::types::auth::Session;

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
    diesel: Db,
    cache_server_manager: CacheServerManager,

    // Cache any computed fields here
    cache_server_manager_cache: Rc<OptionalValue<LuaAnyUserData>>,
    shared_layer_ud: Rc<OptionalValue<LuaAnyUserData>>,
}

impl SharedLayer {
    /// Creates a new SharedLayer
    ///
    /// Should be called once per layer
    #[allow(dead_code)]
    pub fn new(pool: sqlx::PgPool, diesel: Db) -> Self {
        Self {
            cache_server_manager: CacheServerManager::new(pool.clone()),
            pool,
            diesel,
            cache_server_manager_cache: OptionalValue::new().into(),
            shared_layer_ud: OptionalValue::new().into(),
        }
    }

    /// Returns the SharedLayer as LuaUserData
    pub fn as_lua_userdata(&self, lua: &Lua) -> LuaResult<LuaAnyUserData> {
        self.shared_layer_ud
            .get_failable(|| lua.create_userdata(self.clone()))
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

    /// Fetches the session of a entity given token
    pub async fn get_session_by_token(
        &self,
        token: &str,
    ) -> Result<Option<Session>, sqlx::Error> {
        // Delete old/expiring auths first
        sqlx::query("DELETE FROM api_sessions WHERE expiry < NOW()")
            .execute(&self.pool)
            .await?;

        let session: Option<Session> = sqlx::query_as(
            "SELECT id, name, created_at, type AS session_type, target_type, target_id, expiry 
             FROM api_sessions WHERE token = $1 AND expiry >= NOW()",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(session)
    }

    /// Creates a new EntityManager for the given entity type
    pub fn entity_manager_for(&self, target_type: &str) -> Option<crate::entity::AnyEntityManager> {
        let Some(manager) = EntityType::from_name(target_type, self.pool.clone(), self.diesel.clone()) else {
            return None;
        };
        Some(EntityManager::new(manager))
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

        methods.add_scheduler_async_method(
            "GetSessionByToken",
            |lua, this, token: String| async move {
                let session = this
                    .get_session_by_token(&token)
                    .await
                    .map_err(LuaError::external)?;
                lua.to_value(&session)
            },
        );
    }
}
