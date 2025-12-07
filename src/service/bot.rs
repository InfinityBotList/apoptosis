use std::sync::Arc;
use mluau::prelude::*;
use serenity::all::{Cache, GuildId, Http, JsonHttp};
use super::discord::{discordprovider::DiscordProvider, create_discord_tab};

/// A bot instance that can create guild-specific HTTP providers for Lua scripts.
#[allow(dead_code)]
pub struct Bot {
    cache: Arc<Cache>,
    http: Arc<Http>,
    json_http: JsonHttp,
}

#[allow(dead_code)]
impl Bot {
    pub fn new(cache: Arc<Cache>, http: Arc<Http>) -> Self {
        let json_http = JsonHttp::new(http.clone());
        Self {
            cache,
            http,
            json_http,
        }
    }

    pub fn http(&self) -> &Arc<Http> {
        &self.http
    }

    pub fn cache(&self) -> &Arc<Cache> {
        &self.cache
    }

    pub fn json_http(&self) -> &JsonHttp {
        &self.json_http
    }
}

impl LuaUserData for Bot {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("HTTPForGuild", |lua, this, guild_id: String| {
            let guild_id = guild_id.parse::<GuildId>().map_err(|e| LuaError::external(e))?;
            let provider = LuaDiscordProvider {
                guild_id,
                cache: this.cache.clone(),
                json_http: this.json_http.clone(),
            };
            
            create_discord_tab(lua, provider)
        });
    }
}

#[derive(Clone)]
pub struct LuaDiscordProvider {
    guild_id: GuildId,
    cache: Arc<Cache>,
    json_http: JsonHttp
}

impl DiscordProvider for LuaDiscordProvider {
    fn attempt_action(&self, _bucket: &str) -> Result<(), crate::Error> {
        Ok(()) // No internal ratelimiting yet
    }

    fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    fn serenity_http(&self) -> &JsonHttp {
        &self.json_http
    }

    fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        Some(self.cache.current_user().clone())
    }

    fn serenity_cache(&self) -> &serenity::cache::Cache {
        &self.cache
    }
}