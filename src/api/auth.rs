use chrono::{DateTime, Duration, Utc};
use rand::distr::{Alphanumeric, SampleString};
use sqlx::PgPool;

use crate::{entity::{AnyEntityManager, Entity, EntityFlags}, service::sharedlayer::SharedLayer, types::auth::Session};

/// The response from checking web auth
/// 
/// This enum can be used to control API access
pub enum AuthResponse {
    Success {
        session: Session,
        flags: EntityFlags,
        manager: AnyEntityManager,
    },
    ApiBanned {
        session: Session,
    },
    InvalidToken,
    EntityNotSupported,
}

pub async fn check_web_auth(
    shared: &SharedLayer,
    token: &str,
) -> Result<AuthResponse, crate::Error> {
    let sess = shared.get_session_by_token(token)
        .await?;

    let Some(auth) = sess else {
        return Ok(AuthResponse::InvalidToken);
    };

    let Some(manager) = shared.entity_manager_for(&auth.target_type) else {
        return Ok(AuthResponse::EntityNotSupported);
    };

    let flags = manager.entity().flags(&auth.target_id).await?;

    if flags.contains(EntityFlags::BANNED) {
        return Ok(AuthResponse::ApiBanned {
            session: auth,
        });
    }

    // If everything is fine, return the success response
    Ok(AuthResponse::Success {
        session: auth,
        flags,
        manager
    })
}

pub struct ICreatedWebSession {
    pub session_id: String,
    pub token: String,
    pub expires_at: DateTime<Utc>
}

pub enum SessionType {
    Login,
    Api {
        expires_at: DateTime<Utc>,
    }
}

/// 1 hour expiry time
const LOGIN_EXPIRY_TIME: Duration = Duration::seconds(3600);

/// Create a new session
pub async fn create_web_session(
    pool: &PgPool, 
    target_type: &str,
    target_id: &str,
    name: Option<String>,
    session_type: SessionType,
) -> Result<ICreatedWebSession, crate::Error> {
    // Generate a new session ID
    #[derive(sqlx::FromRow)]
    struct NewSession {
        #[sqlx(rename = "id")]
        session_id: uuid::Uuid,
    }

    let token = Alphanumeric.sample_string(&mut rand::rng(), 128);

    let (session_type, expiry) = match session_type {
        SessionType::Login => ("login", Utc::now() + LOGIN_EXPIRY_TIME),
        SessionType::Api { expires_at } => ("api", expires_at),
    };

    let new_session: NewSession = sqlx::query_as(
        "INSERT INTO api_sessions (target_type, target_id, type, token, expiry, name) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(target_type)
    .bind(target_id)
    .bind(session_type)
    .bind(&token)
    .bind(expiry)
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(ICreatedWebSession { 
        session_id: new_session.session_id.to_string(),
        token,
        expires_at: expiry,
    })
}

/// Returns the list of all sessions for a user
pub async fn get_user_sessions(pool: &PgPool, user_id: &str) -> Result<Vec<UserSession>, crate::Error> {
    #[derive(sqlx::FromRow)]
    pub struct UserSessionRow {
        pub id: uuid::Uuid,
        pub name: Option<String>,
        pub user_id: String,
        pub created_at: DateTime<Utc>,
        pub typ: String,
        pub expiry: DateTime<Utc>,
    }

    let sessions: Vec<UserSessionRow> = sqlx::query_as(
        "SELECT id, name, user_id, created_at, type AS typ, expiry FROM web_api_tokens WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let user_sessions = sessions.into_iter().map(|s| UserSession {
        id: s.id.to_string(),
        name: s.name,
        user_id: s.user_id,
        created_at: s.created_at,
        r#type: s.typ,
        expiry: s.expiry,
    }).collect();

    Ok(user_sessions)
}

pub async fn delete_user_session(pool: &PgPool, user_id: &str, session_id: &str) -> Result<(), crate::Error> {
    let session_id_uuid = match uuid::Uuid::parse_str(session_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err("Invalid session ID format".into()),
    };
    
    let res = sqlx::query("DELETE FROM web_api_tokens WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(session_id_uuid)
        .execute(pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err("No session found to delete".into());
    }

    Ok(())
}