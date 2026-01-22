use std::sync::Arc;

use serenity::all::UserId;
use sqlx::PgPool;
use serenity::model::user::OnlineStatus;
use chrono::{DateTime, Utc};
use crate::types::dovewing::PlatformUser;

#[derive(Clone)]
pub enum DovewingSource {
    Discord(serenity::all::Context),
}

impl DovewingSource {
    /// Returns the expiry time of a user
    pub fn user_expiry_time(&self) -> i64 {
        match self {
            // 8 hours
            DovewingSource::Discord(_) => 8 * 60 * 60,
        }
    }

    /// Returns a cached user if available
    pub fn cached_user(&self, user_id: &str) -> Result<Option<PlatformUser>, crate::Error> {
        match self {
            DovewingSource::Discord(c) => {
                let Ok(uid) = user_id.parse::<UserId>() else {
                    return Err("Invalid user id".into());
                };

                for gid in c.cache.guilds() {
                    if let Some(guild) = c.cache.guild(gid) {
                        if let Some(member) = guild.members.get(&uid) {
                            // Check precenses for status
                            let p = {
                                let guild = c.cache.guild(gid);

                                if let Some(guild) = guild {
                                    let p = guild.presences.get(&uid);
                                    p.cloned()
                                } else {
                                    None
                                }
                            };

                            return Ok(Some(PlatformUser {
                                id: user_id.to_string(),
                                username: member.user.name.clone().to_string(),
                                display_name: {
                                    if let Some(ref display_name) = member.user.global_name {
                                        display_name.clone()
                                    } else {
                                        member.user.name.clone()
                                    }
                                }
                                .to_string(),
                                bot: member.user.bot(),
                                avatar: member.user.face(),
                                status: if let Some(p) = p {
                                    match p.status {
                                        OnlineStatus::Online => "online",
                                        OnlineStatus::Idle => "idle",
                                        OnlineStatus::DoNotDisturb => "dnd",
                                        OnlineStatus::Invisible => "invisible",
                                        OnlineStatus::Offline => "offline",
                                        _ => "offline",
                                    }
                                    .to_string()
                                } else {
                                    "offline".to_string()
                                },
                            }));
                        }
                    }
                }

                Ok(None)
            }
        }
    }

    pub async fn http_user(&self, user_id: &str) -> Result<PlatformUser, crate::Error> {
        match self {
            DovewingSource::Discord(c) => {
                let Ok(uid) = user_id.parse::<UserId>() else {
                    return Err("Invalid user id".into());
                };

                let user = uid.to_user(&c.http).await?;

                Ok(PlatformUser {
                    id: user_id.to_string(),
                    username: user.name.clone().to_string(),
                    display_name: {
                        if let Some(ref display_name) = user.global_name {
                            display_name.clone()
                        } else {
                            user.name.clone()
                        }
                    }
                    .to_string(),
                    bot: user.bot(),
                    avatar: user.face(),
                    status: "offline".to_string(),
                })
            }
        }
    }
}

#[derive(Clone)]
pub struct Dovewing {
    pool: PgPool,
    src: DovewingSource,
    middleware: Arc<dyn Fn(PlatformUser) -> Result<PlatformUser, crate::Error> + Send + Sync>,
}

impl Dovewing {
    pub async fn get_platform_user(
        &self,
        user_id: &str,
    ) -> Result<PlatformUser, crate::Error> {
        // First check cache_http
        let cached_uid = self.src.cached_user(user_id)?;

        if let Some(cached_uid) = cached_uid {
            let cached_uid = (self.middleware)(cached_uid)?;
            // Update internal_user_cache__discord
            sqlx::query(
                "INSERT INTO internal_user_cache__discord (id, username, display_name, avatar, bot) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET username = $2, display_name = $3, avatar = $4, bot = $5",
            )
            .bind(user_id)
            .bind(&cached_uid.username)
            .bind(&cached_uid.display_name)
            .bind(&cached_uid.avatar)
            .bind(cached_uid.bot)
            .execute(&self.pool)
            .await?;

            return Ok(cached_uid)
        }

        // Then check internal_user_cache__discord
        #[derive(sqlx::FromRow)]
        pub struct UserCacheRecord {
            username: String,
            display_name: String,
            avatar: String,
            bot: bool,
            last_updated: DateTime<Utc>,
        }

        let rec: Option<UserCacheRecord> = sqlx::query_as("SELECT username, display_name, avatar, bot, last_updated FROM internal_user_cache__discord WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(rec) = rec {
            if rec.last_updated.timestamp() + self.src.user_expiry_time() < chrono::Utc::now().timestamp() {
                // Make a tokio task to update the cache
                let self_ref = self.clone();
                let user_id = user_id.to_string();

                tokio::spawn(async move {
                    let user = self_ref.src.http_user(&user_id).await?;
                    let user = (self_ref.middleware)(user)?;

                    sqlx::query(
                        "UPDATE internal_user_cache__discord SET username = $1, display_name = $2, avatar = $3, bot = $4, last_updated = NOW() WHERE id = $5"
                    )
                    .bind(&user.username)
                    .bind(&user.display_name)
                    .bind(&user.avatar)
                    .bind(user.bot)
                    .bind(&user_id)
                    .execute(&self_ref.pool)
                    .await?;

                    Ok::<(), crate::Error>(())
                });
            }

            let pu = PlatformUser {
                id: user_id.to_string(),
                username: rec.username,
                display_name: rec.display_name,
                bot: rec.bot,
                avatar: rec.avatar,
                status: "offline".to_string(),
            };

            (self.middleware)(pu)
        } else {
            // Fetch from http
            let user = self.src.http_user(user_id).await?;
            let user = (self.middleware)(user)?;

            sqlx::query(
                "INSERT INTO internal_user_cache__discord (id, username, display_name, avatar, bot) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET username = $2, display_name = $3, avatar = $4, bot = $5",
            )
            .bind(user_id)
            .bind(&user.username)
            .bind(&user.display_name)
            .bind(&user.avatar)
            .bind(user.bot)
            .execute(&self.pool)
            .await?;

            Ok(user)
        }
    }
}
